#[derive(Clone, Default)]
pub(crate) struct RenderCounts(
    pub(crate) std::rc::Rc<std::cell::RefCell<std::collections::BTreeMap<String, usize>>>,
);

impl gpui::Global for RenderCounts {}

pub(crate) fn count_render(key: &str, cx: &gpui::App) {
    if let Some(counts) = cx.try_global::<RenderCounts>() {
        *counts.0.borrow_mut().entry(key.to_owned()).or_default() += 1;
    }
}
