use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, SimplePluginCommand};
use nu_protocol::{LabeledError, ShellError, Signature, Value};

use crate::{ArgSignature, FromValues};

pub type BoxedCommand<Plugin> = Box<dyn PluginCommand<Plugin = Plugin>>;

type RunFn<Plugin> = Box<
    dyn Fn(&Plugin, &EngineInterface, &EvaluatedCall) -> Result<Value, ShellError> + Send + Sync,
>;

pub struct Command<Plugin> {
    name: &'static str,
    description: &'static str,
    arg_signatures: Vec<ArgSignature>,
    run_fn: RunFn<Plugin>,
}

impl<Plugin> Command<Plugin>
where
    Plugin: nu_plugin::Plugin + 'static,
{
    pub fn new<Args>(
        name: &'static str,
        description: &'static str,
        run_fn: impl Fn(&Plugin, &EngineInterface, Args::Output<'_>) -> Result<Value, ShellError>
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

    pub fn boxed(self) -> Box<dyn PluginCommand<Plugin = Plugin>> {
        Box::new(self)
    }
}

impl<Plugin> From<Command<Plugin>> for Box<dyn PluginCommand<Plugin = Plugin>>
where
    Plugin: nu_plugin::Plugin + 'static,
{
    fn from(command: Command<Plugin>) -> Self {
        Box::new(command)
    }
}

impl<Plugin> SimplePluginCommand for Command<Plugin>
where
    Plugin: nu_plugin::Plugin,
{
    type Plugin = Plugin;

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
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let value = (self.run_fn)(plugin, engine, call)?;

        Ok(value)
    }
}
