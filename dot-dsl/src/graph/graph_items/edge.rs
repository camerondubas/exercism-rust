use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct Edge {
    a: String,
    b: String,
    attrs: HashMap<String, String>
}

impl Edge {
    pub fn new(a: &str, b: &str) -> Self {
        Edge {
            a: a.into(),
            b: b.into(),
            attrs: HashMap::new()
        }
    }

    pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
        Edge {
            attrs: attrs
                .iter()
                .map(|(k,v)| (String::from(*k), String::from(*v)))
                .collect(),
            ..self
        }
    }
}
