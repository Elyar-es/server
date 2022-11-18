use std::collections::HashMap;

pub struct QueryString<'a> {
    data: HashMap<&'a str, Value<'a>>,
}
 
pub enum Value<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
}

impl<'a> QueryString<'a> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'a> From<&'a str> for QueryString<'a> {
    fn from(s: &'a str) -> Self {

        let mut data  = HashMap::new();

        for sub in s.split('&') {
            let mut key = sub;
            let mut val = "";
            if let Some(i) = sub.find('=') {
                key = &sub[..i];
                val = &sub[i+1..];
            }

            data.entry(key)
            .and_modify(|existing| match existing {
                Value::Single(prev_val) => {
                    *existing = Value::Multiple(vec![prev_val, val]);
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


