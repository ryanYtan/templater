use unicode_segmentation::UnicodeSegmentation;

use crate::err::TemplateError;


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TemplateElement {
    Part {
        literal: String
    },
    Replace {
        selector: String
    },
}

pub struct Formatter {
    pub(crate) elements: Vec<TemplateElement>
}

impl IntoIterator for Formatter {
    type Item = TemplateElement;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl Formatter {
    /// Rules:
    /// "..." refers to any string (including empty string) WITHOUT a closed rounded parenthesis ")"
    ///
    /// 1. "%%" is treated as a literal "%"
    /// 2. "%" followed by any character that is not "(" is invalid
    /// 3. "%(...)" is always valid
    /// 4. "%(...)x" where "x" is any string is valid
    /// 5. "%(..." is invalid
    pub fn build<S>(fmts: S) -> Result<Self, TemplateError>
        where S: Into<String>
    {
        let as_str = fmts.into();
        let mut chars = as_str.graphemes(true);
        let mut result = Vec::new();
        let mut tmp = String::new();

        while let Some(c) = chars.next() {
            if c != "%" {
                tmp.push_str(c);
                continue;
            }

            match chars.next() {
                Some("%") => {
                    tmp.push_str("%")
                },
                Some("(") => {
                    if !tmp.is_empty() {
                        result.push(TemplateElement::Part { literal: tmp.clone() });
                    }
                    tmp.clear();
                    let mut selector = String::new();
                    loop {
                        match chars.next() {
                            Some(")") => break,
                            Some(c) => selector.push_str(c),
                            None => return Err(TemplateError::UnexpectedEnd { formats: as_str }),
                        }
                    }
                    result.push(TemplateElement::Replace { selector });
                },
                Some(c) => return Err(TemplateError::UnexpectedCharacter {
                    character: c.to_string(),
                    formats: as_str,
                }),
                None => return Err(TemplateError::UnexpectedEnd { formats: as_str }),
            };
        }

        if !tmp.is_empty() {
            result.push(TemplateElement::Part { literal: tmp.clone() });
        }

        Ok(Self { elements: result })
    }
}

#[cfg(test)]
mod tests {
    use crate::formatter::TemplateElement;
    use crate::err::TemplateError;
    use super::Formatter;

    fn repl(s: &str) -> TemplateElement {
        TemplateElement::Replace { selector: s.to_owned() }
    }

    fn part(s: &str) -> TemplateElement {
        TemplateElement::Part { literal: s.to_owned() }
    }

    #[test]
    fn test_valid() {
        let fmts = "%(title) %(id) %%(select)   ";
        let result = Formatter::build(fmts);
        assert!(result.is_ok());
        let formatter = result.ok().unwrap();
        assert_eq!(formatter.elements, vec![
            repl("title"),
            part(" "),
            repl("id"),
            part(" %(select)   "),
        ])
    }

    #[test]
    fn test_valid_nonalnum_in_selector() {
        let fmts = "%(a.b.c.d)%(p//  q)%((?;:'))";
        let result = Formatter::build(fmts);
        assert!(result.is_ok());
        let formatter = result.ok().unwrap();
        assert_eq!(formatter.elements, vec![
            repl("a.b.c.d"),
            repl("p//  q"),
            repl("(?;:'"),
            part(")"),
        ])
    }

    #[test]
    fn test_valid_marker_literal_simple() {
        let fmts = "%%%%";
        let result = Formatter::build(fmts);
        assert!(result.is_ok());
        let formatter = result.ok().unwrap();
        assert_eq!(formatter.elements, vec![
            part("%%"),
        ])
    }

    #[test]
    fn test_valid_marker_literal_complex() {
        let fmts = "%%%%%%%(a)P%%%%";
        let result = Formatter::build(fmts);
        assert!(result.is_ok());
        let formatter = result.ok().unwrap();
        assert_eq!(formatter.elements, vec![
            part("%%%"),
            repl("a"),
            part("P%%"),
        ])
    }

    #[test]
    fn test_invalid_marker_not_followed_by_open_paren() {
        let fmts = "%%()%l";
        let result = Formatter::build(fmts);
        assert!(result.is_err());
        assert!(matches!(result, Err(TemplateError::UnexpectedCharacter { .. })))
    }

    #[test]
    fn test_invalid_unterminated_selector() {
        let fmts = "%%%(abcd pl";
        let result = Formatter::build(fmts);
        assert!(result.is_err());
        assert!(matches!(result, Err(TemplateError::UnexpectedEnd { .. })))
    }

    #[test]
    fn test_invalid_end_reached_after_marker() {
        let fmts = "%%%%%";
        let result = Formatter::build(fmts);
        assert!(result.is_err());
        assert!(matches!(result, Err(TemplateError::UnexpectedEnd { .. })))
    }
}
