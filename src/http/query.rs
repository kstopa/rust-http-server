use std::{collections::HashMap, fmt::Write};

/// Store params in a query string with repeated param names
/// that are stored in an array.
#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                // Find the first occurrence of '='
                key = &sub_str[..i]; // Everything before '=' is the key
                val = &sub_str[i + 1..]; // Everything after '=' is the value
            }
            // Check if is a Single or Multiple value
            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        // let mut vec = Vec::new();
                        // vec.push(prev_val);
                        // vec.push(val);
                        // Same as VVVVV using a vec! macro
                        let vec = vec![prev_val, val];
                        *existing = Value::Multiple(vec);
                    }
                    Value::Multiple(vec) => {
                        vec.push(val);
                    }
                })
                .or_insert(Value::Single(val));
        }
        QueryString { data }
    }
}
