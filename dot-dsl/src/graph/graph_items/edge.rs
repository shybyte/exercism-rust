#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    from: String,
    to: String,
    attrs: Vec<(String, String)>,
}

impl Edge {
    pub fn new(from_id: &str, to_id: &str) -> Self {
        Self {
            from: from_id.to_string(),
            to: to_id.to_string(),
            attrs: vec![],
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs = attrs.iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }
}

