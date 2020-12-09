pub type Content = Box<dyn FnOnce() -> Box<dyn Iterator<Item = String>> + Send + Sync>;
