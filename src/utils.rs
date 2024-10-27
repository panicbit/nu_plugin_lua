use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, ShellError, Signature, Span, SyntaxShape};
use std::marker::PhantomData;

use crate::extensions::{ArgSliceExt, FromArg};
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

struct ArgSignature {
    name: &'static str,
    syntax_shape: SyntaxShape,
    description: &'static str,
}

pub struct CommandBuilder<Args> {
    name: &'static str,
    description: &'static str,
    arg_signatures: Vec<ArgSignature>,
    _p: PhantomData<Args>,
}

impl CommandBuilder<()> {
    pub fn new(name: &'static str, description: &'static str) -> Self {
        Self {
            name,
            description,
            arg_signatures: Vec::new(),
            _p: PhantomData,
        }
    }
}

impl<ARGS> CommandBuilder<ARGS> {
    pub fn arg<T: FromArg>(
        mut self,
        name: &'static str,
        description: &'static str,
    ) -> CommandBuilder<<ARGS as Append<T>>::Output>
    where
        ARGS: Append<T>,
    {
        self.arg_signatures.push(ArgSignature {
            name,
            syntax_shape: T::syntax_shape(),
            description,
        });

        CommandBuilder {
            name,
            description,
            arg_signatures: self.arg_signatures,
            _p: PhantomData,
        }
    }

    pub fn run<F>(self, f: F) -> Command<ARGS, F>
    where
        ARGS: Args,
        F: Fn(&crate::Plugin, &EngineInterface, ARGS::Parsed<'_>) -> Result<NuValue, ShellError>
            + Send
            + Sync,
        F: Send + Sync,
    {
        Command {
            name: self.name,
            description: self.description,
            arg_signatures: self.arg_signatures,
            run_fn: f,
            _p: PhantomData,
        }
    }
}

pub trait Args {
    type Parsed<'a>;

    fn from_values(values: &[NuValue]) -> Result<Self::Parsed<'_>, ShellError>;
}

impl Args for () {
    type Parsed<'a> = ();

    fn from_values(_values: &[NuValue]) -> Result<Self::Parsed<'_>, ShellError> {
        Ok(())
    }
}

impl<A0> Args for (A0,)
where
    A0: FromArg,
{
    type Parsed<'a> = (A0::Output<'a>,);

    fn from_values(values: &[NuValue]) -> Result<Self::Parsed<'_>, ShellError> {
        Ok((values.arg::<A0>(0)?,))
    }
}

impl<A0, A1> Args for (A0, A1)
where
    A0: FromArg,
    A1: FromArg,
{
    type Parsed<'a> = (A0::Output<'a>, A1::Output<'a>);

    fn from_values(values: &[NuValue]) -> Result<Self::Parsed<'_>, ShellError> {
        Ok((values.arg::<A0>(0)?, values.arg::<A1>(1)?))
    }
}

pub trait Append<T> {
    type Output;
}

impl<T> Append<T> for () {
    type Output = (T,);
}

impl<C0, T> Append<T> for (C0,) {
    type Output = (C0, T);
}

pub struct Command<ARGS, F> {
    name: &'static str,
    description: &'static str,
    arg_signatures: Vec<ArgSignature>,
    run_fn: F,
    _p: PhantomData<ARGS>,
}

impl<ARGS, F> Command<ARGS, F>
where
    ARGS: Args + Send + Sync + 'static,
    F: Fn(&crate::Plugin, &EngineInterface, ARGS::Parsed<'_>) -> Result<NuValue, ShellError>,
    F: Send + Sync + 'static,
{
    pub fn boxed(self) -> Box<dyn nu_plugin::PluginCommand<Plugin = crate::Plugin>> {
        Box::new(self)
    }
}

impl<ARGS, F> SimplePluginCommand for Command<ARGS, F>
where
    ARGS: Args + Send + Sync,
    F: Fn(&crate::Plugin, &EngineInterface, ARGS::Parsed<'_>) -> Result<NuValue, ShellError>,
    F: Send + Sync,
{
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
        let args = ARGS::from_values(&call.positional)?;
        let value = (self.run_fn)(plugin, engine, args)?;

        Ok(value)
    }
}
