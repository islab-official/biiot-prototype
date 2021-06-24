use std::collections::HashMap;

pub struct HttpHeader(HashMap<String, String>);

impl Default for HttpHeader {
    fn default() -> Self {
        HttpHeader { 0: HashMap::new() }
    }
}

impl From<HashMap<String, String>> for HttpHeader {
    fn from(value: HashMap<String, String>) -> Self {
        HttpHeader { 0: value }
    }
}

impl Clone for HttpHeader {
    fn clone(&self) -> Self {
        let mut map = HashMap::new();
        for kv in self.0.iter() {
            map.insert(kv.0.to_string(), kv.1.to_string());
        }
        HttpHeader { 0: map }
    }
}

impl ToString for HttpHeader {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for kv in self.0.iter() {
            result.push_str(kv.0.as_str());
            result.push_str(": ");
            result.push_str(kv.1.as_str());
            result.push_str("\r\n");
        }
        result.push_str("\r\n");
        result
    }
}

impl HttpHeader {
    pub fn headers(&self) -> Vec<(String, String)> {
        let mut v = vec![];
        for kv in self.0.iter() {
            v.push((kv.0.clone(), kv.1.clone()));
        }
        return v;
    }

    pub fn add_header(&mut self, key: &str, value: &str) -> Result<(), ()> {
        if self.0.contains_key(key) {
            return Err(());
        }
        self.0.insert(key.to_string(), value.to_string());
        return Ok(());
    }

    pub fn remove_header(&mut self, key: &str) -> bool {
        if self.0.contains_key(key) {
            self.0.remove(key);
            return true;
        }
        return false;
    }
}