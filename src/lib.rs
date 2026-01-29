#[derive(Default)]
pub struct Spreadpages {
    path: String,
}

impl Spreadpages {
    pub fn new(path: &str) -> Self {
        let mut sp = Self.default();
        sp.path = path.to_string();
        sp
    }
}
