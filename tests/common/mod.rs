use std::collections::BTreeMap;

#[derive(Debug)]
pub(crate) struct ComplexObject {
    pub(crate) a: i64,
    pub(crate) b: String,
    pub(crate) c: Option<String>,
    pub(crate) d: Vec<String>,
    pub(crate) e: ComplexSubObject,
}

#[derive(Debug)]
pub struct ComplexSubObject {
    pub(crate) f: (i32, i32),
    pub(crate) g: Vec<Option<i32>>,
    pub(crate) h: BTreeMap<String, i32>,
    pub(crate) i: Option<String>,
}

pub(crate) fn create_complex() -> ComplexObject {
    ComplexObject {
        a: 64,
        b: "a string".to_owned(),
        c: Some("optional".to_owned()),
        d: vec!["first".to_owned(), "second".to_owned()],
        e: ComplexSubObject {
            f: (-2147, 2147),
            g: vec![],
            h: (|| {
                let mut map = BTreeMap::new();
                map.insert("hundred".to_owned(), 100);
                map.insert("thousand".to_owned(), 1000);
                map.insert("ten thousand".to_owned(), 10000);
                map
            })(),
            i: None,
        }
    }
}
