use nu_plugin::{PluginCommand, SimplePluginCommand};
use nu_protocol::{LabeledError, Signature, Span};

use crate::custom::PluginValue;
use crate::utils::Command;
use crate::NuValue;

pub fn new() -> Box<dyn PluginCommand<Plugin = crate::Plugin>> {
    Command::new::<_, ()>(
        "lua new",
        "create a new lua instance",
        |plugin, engine, ()| {
            engine.set_gc_disabled(true)?;

            let span = Span::unknown();
            let (lua, _) = plugin.create_lua();
            let plugin_value = PluginValue::Lua(lua);
            let value = NuValue::custom(Box::new(plugin_value), span);

            Ok(value)
        },
    )
    .into()
}

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
        let (lua, _) = plugin.create_lua();
        let plugin_value = PluginValue::Lua(lua);
        let value = NuValue::custom(Box::new(plugin_value), span);

        Ok(value)
    }
}
