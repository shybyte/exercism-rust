use std::collections::BTreeMap;

pub struct School {
    db: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School { db: BTreeMap::new() }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.db.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Option<&Vec<String>> {
        self.db.get(&grade)
    }

    pub fn add(&mut self, grade: u32, name: &str) {
        let names = self.db.entry(grade).or_insert(vec![]);
        names.push(name.to_string());
        names.sort();
    }
}
