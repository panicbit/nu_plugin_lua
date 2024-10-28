use nu_plugin::PluginCommand;

use crate::custom;
use crate::utils::Command;
use nu_plugin_helpers::FromValues;

#[derive(FromValues)]
struct Args<'a> {
    /// lua instance
    /// test D:
    lua: &'a custom::Lua,
    /// lua code to evaluate
    code: &'a str,
}

pub fn eval() -> Box<dyn PluginCommand<Plugin = crate::Plugin>> {
    Command::new::<Args>(
        "lua eval",
        "evaluate lua to a nushell value",
        |plugin, _engine, Args { lua, code }| plugin.eval_lua(lua, code),
    )
    .into()
}
