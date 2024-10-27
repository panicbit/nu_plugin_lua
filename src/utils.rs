use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, SimplePluginCommand};
use nu_protocol::{LabeledError, ShellError, Signature, Span, SyntaxShape};

use crate::NuValue;

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

pub trait FromValues {
    type Output<'a>;

    fn from_values(positional: &[NuValue]) -> Result<Self::Output<'_>, ShellError>;
    fn arg_signatures() -> Vec<ArgSignature>;
}

impl FromValues for () {
    type Output<'a> = ();

    fn from_values(_positional: &[NuValue]) -> Result<Self::Output<'_>, ShellError> {
        Ok(())
    }

    fn arg_signatures() -> Vec<ArgSignature> {
        vec![]
    }
}

type RunFn = Box<
    dyn Fn(&crate::Plugin, &EngineInterface, &EvaluatedCall) -> Result<NuValue, ShellError>
        + Send
        + Sync,
>;

pub struct Command {
    name: &'static str,
    description: &'static str,
    arg_signatures: Vec<ArgSignature>,
    run_fn: RunFn,
    // https://doc.rust-lang.org/nomicon/phantom-data.html#table-of-phantomdata-patterns
}

impl Command {
    pub fn new<F, Args>(name: &'static str, description: &'static str, run_fn: F) -> Self
    where
        F: Fn(&crate::Plugin, &EngineInterface, Args::Output<'_>) -> Result<NuValue, ShellError>,
        F: Send + Sync + 'static,
        Args: FromValues,
    {
        Self {
            name,
            description,
            arg_signatures: Args::arg_signatures(),
            run_fn: Box::new(move |plugin, engine, call| {
                let args = Args::from_values(&call.positional)?;
                run_fn(plugin, engine, args)
            }),
        }
    }
}

impl From<Command> for Box<dyn PluginCommand<Plugin = crate::Plugin>> {
    fn from(command: Command) -> Self {
        Box::new(command)
    }
}

impl SimplePluginCommand for Command {
    type Plugin = crate::Plugin;

    fn name(&self) -> &str {
        self.name
    }

    fn description(&self) -> &str {
        self.description
    }

    fn signature(&self) -> nu_protocol::Signature {
        let mut sig = Signature::new(self.name).add_help();

        for arg in &self.arg_signatures {
            sig = sig.required(arg.name, arg.syntax_shape.clone(), arg.description);
        }

        sig
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &NuValue,
    ) -> Result<NuValue, LabeledError> {
        let value = (self.run_fn)(plugin, engine, call)?;

        Ok(value)
    }
}
