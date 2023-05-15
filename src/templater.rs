use std::{collections::HashMap};

use crate::err::TemplateError;
use crate::formatter::{TemplateElement, Formatter};

pub struct Templater<T> {
    mapping: HashMap<String, Box<dyn Fn(&T) -> Option<String> + 'static + Send + Sync>>
}

impl<T> Templater<T> {
    pub fn new() -> Self {
        Templater {
            mapping: HashMap::new(),
        }
    }

    pub fn insert<S, F>(&mut self, selector: S, accessor: F)
        where
            S: Into<String>,
            F: Fn(&T) -> Option<String> + 'static + Send + Sync
    {
        self.mapping.insert(selector.into(), Box::new(accessor));
    }

    pub fn extend<I, S, U>(&mut self, selectors: I)
        where
            I: IntoIterator<Item = (S, U)>,
            S: Into<String>,
            U: (Fn(&T) -> Option<String>) + 'static + Send + Sync
    {
        for (selector, accessor) in selectors {
            self.insert(selector, accessor);
        }
    }

    pub fn remove<S>(&mut self, selector: S) -> bool
        where S: Into<String>
    {
        self.mapping.remove(&selector.into()).is_some()
    }

    pub fn renderf(&self, obj: &T, formatter: &Formatter) -> Result<String, TemplateError> {
        let strings: Vec<String> = formatter
            .elements
            .iter()
            .map(|e| {
                match e {
                    TemplateElement::Part { literal } => {
                        Ok(literal.clone())
                    }
                    TemplateElement::Replace { selector } => {
                        let accessor = self.mapping.get(selector);

                        // the lack of an accessor associated with a particular
                        // selector is invalid, however the value accessed by the
                        // accessor might not exist, in which case the placeholder
                        // "NA" is used
                        match accessor {
                            Some(f) => {
                                let out = f(&obj).or(Some("NA".to_owned())).unwrap();
                                Ok(out)
                            },
                            None => Err(TemplateError::UnknownSelector {
                                selector: selector.clone(),
                            }),
                        }
                    }
                }
            })
            .collect::<Result<Vec<String>, TemplateError>>()?;

        Ok(strings
            .into_iter()
            .reduce(|a, b| a + &b)
            .or(Some("".to_owned()))
            .unwrap())
    }

    pub fn render<S>(&self, obj: &T, fmts: S) -> Result<String, TemplateError>
        where S: Into<String>
    {
        let formatter = Formatter::build(fmts.into())?;
        self.renderf(obj, &formatter)
    }
}
