use nu_plugin::PluginCommand;
use nu_plugin_helpers::{Command, FromValues};
use nu_protocol::Span;

use crate::custom::PluginValue;
use crate::NuValue;

#[derive(FromValues)]
struct Args {}

pub fn new() -> Box<dyn PluginCommand<Plugin = crate::Plugin>> {
    Command::<crate::Plugin>::new::<Args>(
        "lua new",
        "create a new lua instance",
        |plugin, engine, _args| {
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
