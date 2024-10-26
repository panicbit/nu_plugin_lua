use nu_plugin::SimplePluginCommand;
use nu_protocol::{LabeledError, Signature, Span, Value as NuValue};

pub struct New;

impl SimplePluginCommand for New {
    type Plugin = crate::Plugin;

    fn name(&self) -> &str {
        "lua new"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::new("lua new").add_help()
    }

    fn description(&self) -> &str {
        "create a new lua instance"
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        engine: &nu_plugin::EngineInterface,
        _call: &nu_plugin::EvaluatedCall,
        _input: &NuValue,
    ) -> Result<NuValue, LabeledError> {
        engine.set_gc_disabled(true)?;

        let span = Span::unknown();
        let lua_handle = plugin.create_lua();
        let value = NuValue::custom(Box::new(lua_handle), span);

        Ok(value)
    }
}
