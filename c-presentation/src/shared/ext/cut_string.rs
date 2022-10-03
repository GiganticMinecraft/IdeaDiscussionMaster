pub trait CutString {
    fn cut_at(&self, index: usize) -> &str;
}

impl CutString for String {
    fn cut_at(&self, index: usize) -> &str {
        match self.char_indices().nth(index) {
            Some((idx, _)) => &self[..idx],
            None => self,
        }
    }
}
