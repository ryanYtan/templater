use crate::templater::Templater;

pub struct TemplaterBuilder<T> {
    t: Templater<T>,
}

impl<T> TemplaterBuilder<T> {
    pub fn new() -> Self {
        Self { t: Templater::new() }
    }

    pub fn with_selector<S, F>(mut self, selector: S, accessor: F) -> Self
        where
            S: Into<String>,
            F: (Fn(&T) -> Option<String>) + 'static + Send + Sync
    {
        self.t.insert(selector.into(), Box::new(accessor));
        self
    }

    pub fn build(self) -> Templater<T> {
        self.t
    }
}
