use serde_json::Value;

#[derive(Copy, Clone, Eq, PartialEq)]
enum BodyDataType { PlainText, ApplicationJson, UrlEncodedForm, MultipartForm }

pub struct UrlEncodedForm(String);
pub struct MultipartForm(String);

pub struct HttpBody(
    BodyDataType,
    String
);

impl Clone for HttpBody {
    fn clone(&self) -> Self {
        HttpBody { 0: self.0.clone(), 1: self.1.clone() }
    }
}

impl Default for HttpBody {
    fn default() -> Self {
        HttpBody {
            0: BodyDataType::PlainText,
            1: "".to_string()
        }
    }
}

impl From<&str> for HttpBody {
    fn from(data: &str) -> Self {
        HttpBody {
            0: BodyDataType::PlainText,
            1: data.to_string()
        }
    }
}

// impl From<HashMap<String, Box<dyn std::any::Any>>> for HttpBody {
//     fn from(data: HashMap<String, Box<dyn Any>>) -> Self {
//         todo!()
//     }
// }

impl From<serde_json::Value> for HttpBody {
    fn from(data: Value) -> Self {
        HttpBody {
            0: BodyDataType::ApplicationJson,
            1: data.to_string()
        }
    }
}

impl ToString for HttpBody {
    fn to_string(&self) -> String {
        self.1.to_string()
    }
}

impl HttpBody {

    pub fn append_str(&mut self, data: String) {
        self.1.push_str(data.as_str());
    }

    pub fn append_json(&mut self, data: serde_json::Value) {
        self.1.push_str(data.to_string().as_str());
    }

    pub fn set_data(&mut self, data: &str) {
        self.1 = data.to_string();
    }

    pub fn data(&self) -> &str {
        self.1.as_str()
    }

    pub fn clear(&mut self) {
        self.1.clear();
    }
}