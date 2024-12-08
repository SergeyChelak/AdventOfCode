pub fn remove_first_and_last(s: &str) -> &str {
    let mut iter = s.chars();
    iter.next();
    iter.next_back();
    iter.as_str()
}

#[cfg(test)]
pub trait AsVecString {
    fn as_strings(&self) -> Vec<String>;
}

#[cfg(test)]
impl AsVecString for [&str] {
    fn as_strings(&self) -> Vec<String> {
        self.iter().map(|s| s.to_string()).collect::<Vec<_>>()
    }
}
