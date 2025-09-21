pub fn remove_first_and_last(s: &str) -> &str {
    let mut iter = s.chars();
    iter.next();
    iter.next_back();
    iter.as_str()
}

pub fn not_found() -> String {
    "Not found".to_string()
}
