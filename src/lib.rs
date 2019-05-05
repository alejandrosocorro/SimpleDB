use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct SimpleDB {
    database: HashMap<String, String>
}

impl SimpleDB {
    fn new() -> SimpleDB {
        let db = HashMap::new();
        SimpleDB { database: db }
    }

    fn set(&mut self, key: &str, value: &str) {
        self.database.insert(key.to_string(), value.to_string());
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.database.get(key)
    }

    fn delete(&mut self, key: &str) -> Option<String> {
        self.database.remove(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_from_db() {
        let mut db = SimpleDB::new();

        db.set("Alejandro", "32");
        db.set("Andrea", "32");
        db.set("Lily", "3");
        db.set("Milo", "8");

        assert_eq!(db.get("Alejandro"), Some(&"32".to_string()));
        assert_eq!(db.get("Bad test"), None);
        assert_eq!(db.database.len(), 4);
    }
}
