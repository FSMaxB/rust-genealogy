pub type Content = Box<dyn FnOnce() -> dyn Iterator<Item = String>>;
