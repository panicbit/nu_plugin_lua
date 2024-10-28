use nu_protocol::{ShellError, SyntaxShape, Value};

pub trait FromValue: Sized {
    type Output<'a>;

    fn from_value(value: &Value) -> Result<Self::Output<'_>, ShellError>;
    fn syntax_shape() -> SyntaxShape;
}

impl FromValue for &'_ str {
    type Output<'a> = &'a str;

    fn from_value(value: &Value) -> Result<Self::Output<'_>, ShellError> {
        value.as_str()
    }

    fn syntax_shape() -> SyntaxShape {
        SyntaxShape::String
    }
}
