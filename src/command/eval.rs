use nu_plugin::PluginCommand;

use crate::custom;
use crate::utils::CommandBuilder;

pub fn eval() -> Box<dyn PluginCommand<Plugin = crate::Plugin>> {
    CommandBuilder::new("lua eval", "evaluate lua to a nushell value")
        .arg::<&custom::Lua>("lua", "the lua state")
        .arg::<&str>("code", "the lua code to evaluate")
        .run(|plugin, _, (lua, code)| plugin.eval_lua(lua, code))
        .boxed()
}
