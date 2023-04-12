use std::collections::HashMap;

pub struct String2IdMapper {
    map: HashMap<String, usize>,
}

impl String2IdMapper {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn get_id(&mut self, s: &str) -> usize {
        let next_id = self.map.len();
        if let Some(id) = self.map.get(s) {
            *id
        } else {
            self.map.insert(s.to_string(), next_id);
            next_id
        }
    }
}
