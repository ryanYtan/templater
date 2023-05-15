use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("Unexpected character \"{character}\" encountered in format string \"{formats}\"")]
    UnexpectedCharacter {
        character: String,
        formats: String,
    },
    #[error("Unexpected end reached in format string \"{formats}\"")]
    UnexpectedEnd {
        formats: String,
    },
    #[error("Unknown selector \"{selector}\"")]
    UnknownSelector {
        selector: String,
    },
}
