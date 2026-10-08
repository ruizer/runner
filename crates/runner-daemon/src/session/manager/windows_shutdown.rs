use std::sync::{Condvar, Mutex};
use std::time::Duration;

use crate::repo::session::SessionRowDb;

#[derive(Default, PartialEq)]
enum State {
    #[default]
    Running,
    Querying,
    Ending,
    Finished(bool),
}

#[derive(Default)]
pub(crate) struct SessionEnd {
    state: Mutex<Coordination>,
    changed: Condvar,
}

#[derive(Default)]
struct Coordination {
    phase: State,
    active_exits: usize,
    exited: Vec<SessionRowDb>,
}

impl SessionEnd {
    pub(crate) fn query(&self) {
        let mut state = self.state.lock().unwrap();
        if state.phase == State::Running {
            state.phase = State::Querying;
        }
    }

    pub(crate) fn cancel(&self) {
        let mut state = self.state.lock().unwrap();
        if state.phase == State::Querying {
            state.phase = State::Running;
            state.exited.clear();
            self.changed.notify_all();
        }
    }

    pub(crate) fn confirm(&self) {
        let mut state = self.state.lock().unwrap();
        if !matches!(state.phase, State::Finished(_)) {
            state.phase = State::Ending;
        }
    }

    pub(crate) fn finish(&self, committed: bool) {
        self.state.lock().unwrap().phase = State::Finished(committed);
        self.changed.notify_all();
    }

    pub(crate) fn wait_for_stamp(&self, timeout: Duration) -> bool {
        let (state, _) = self
            .changed
            .wait_timeout_while(self.state.lock().unwrap(), timeout, |state| {
                !matches!(state.phase, State::Finished(_))
            })
            .unwrap();
        state.phase == State::Finished(true)
    }

    pub(crate) fn exit(&self) -> ExitGuard<'_> {
        let mut state = self
            .changed
            .wait_while(self.state.lock().unwrap(), |state| {
                matches!(state.phase, State::Querying | State::Ending)
            })
            .unwrap();
        state.active_exits += 1;
        ExitGuard {
            session_end: self,
            preserve_key: matches!(state.phase, State::Finished(_)),
            snapshot: None,
        }
    }

    pub(crate) fn take_exited_sessions(&self) -> Vec<SessionRowDb> {
        let mut state = self
            .changed
            .wait_while(self.state.lock().unwrap(), |state| state.active_exits > 0)
            .unwrap();
        std::mem::take(&mut state.exited)
    }
}

// Never hold the coordination mutex over database work: Windows must be able
// to deliver QUERYENDSESSION even while an exit waits for a connection/write.
pub(crate) struct ExitGuard<'a> {
    session_end: &'a SessionEnd,
    preserve_key: bool,
    pub(crate) snapshot: Option<SessionRowDb>,
}

impl ExitGuard<'_> {
    pub(crate) fn preserve_key(&self) -> bool {
        self.preserve_key
    }
}

impl Drop for ExitGuard<'_> {
    fn drop(&mut self) {
        let mut state = self.session_end.state.lock().unwrap();
        state.active_exits -= 1;
        if matches!(state.phase, State::Querying | State::Ending) {
            if let Some(row) = self.snapshot.take() {
                if row.status == crate::model::SessionStatus::Running {
                    state.exited.push(row);
                }
            }
        }
        self.session_end.changed.notify_all();
    }
}
