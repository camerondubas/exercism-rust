use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    name: String,
    attrs: HashMap<String, String>
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            attrs: HashMap::new()
        }
    }

    pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
        Node {
            attrs: attrs
                .iter()
                .map(|(k,v)| (String::from(*k), String::from(*v)))
                .collect(),
            ..self
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_attr(&self, attr: &str) -> Option<&str> {
        self.attrs.get(attr).map(String::as_str)
    }
}
