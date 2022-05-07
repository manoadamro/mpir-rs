#[derive(Debug)]
pub struct ParseMpzError {
    _priv: (),
}

impl Default for ParseMpzError {
    fn default() -> Self {
        Self { _priv: () }
    }
}
