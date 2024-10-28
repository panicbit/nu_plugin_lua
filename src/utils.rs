use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, SimplePluginCommand};
use nu_plugin_helpers::{ArgSignature, FromValues};
use nu_protocol::{LabeledError, ShellError, Signature, Span};

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
}

impl Command {
    pub fn new<Args>(
        name: &'static str,
        description: &'static str,
        run_fn: impl Fn(&crate::Plugin, &EngineInterface, Args::Output<'_>) -> Result<NuValue, ShellError>
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Args: FromValues,
    {
        Self {
            name,
            description,
            run_fn: Box::new(move |plugin, engine, call| {
                let args = Args::from_values(&call.positional)?;
                run_fn(plugin, engine, args)
            }),
            arg_signatures: Args::arg_signatures(),
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
