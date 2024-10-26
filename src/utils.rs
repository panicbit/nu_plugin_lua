use nu_protocol::{LabeledError, ShellError, Span};

pub fn type_error(type_name: impl AsRef<str>, span: Span) -> ShellError {
    let type_name = type_name.as_ref();

    ShellError::TypeMismatch {
        err_message: format!("expected {type_name}"),
        span,
    }
}

pub fn simple_error(msg: impl Into<String>) -> ShellError {
    LabeledError::new(msg).into()
}
