pub struct AlnumChecker;

impl AlnumChecker {
    pub fn new() -> Self {
        AlnumChecker
    }

    pub fn is_alnum(&self, c: char) -> bool {
        c.is_ascii_alphanumeric()
    }

    pub fn is_match(&self, text: &str) -> bool {
        text.chars().all(|c| self.is_alnum(c))
    }
}
