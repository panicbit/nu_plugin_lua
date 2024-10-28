use nu_protocol::{ShellError, SyntaxShape, Value};

pub trait FromValues {
    type Output<'a>;

    fn from_values(positional: &[Value]) -> Result<Self::Output<'_>, ShellError>;
    fn arg_signatures() -> Vec<ArgSignature>;
}

impl FromValues for () {
    type Output<'a> = ();

    fn from_values(_positional: &[Value]) -> Result<Self::Output<'_>, ShellError> {
        Ok(())
    }

    fn arg_signatures() -> Vec<ArgSignature> {
        vec![]
    }
}

pub struct ArgSignature {
    pub name: &'static str,
    pub description: &'static str,
    pub syntax_shape: SyntaxShape,
}

impl ArgSignature {
    pub fn new(name: &'static str, description: &'static str, syntax_shape: SyntaxShape) -> Self {
        Self {
            name,
            description,
            syntax_shape,
        }
    }
}
