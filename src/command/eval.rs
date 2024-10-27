use nu_plugin::PluginCommand;
use nu_protocol::ShellError;

use crate::extensions::{ArgSliceExt, FromValue};
use crate::utils::{ArgSignature, Command, FromValues};
use crate::{custom, NuValue};

struct Args<'a> {
    // lua instance
    lua: &'a custom::Lua,
    // lua code to evaluate
    code: &'a str,
}

impl FromValues for Args<'_> {
    type Output<'a> = Args<'a>;

    fn from_values(positional: &[NuValue]) -> Result<Self::Output<'_>, ShellError> {
        Ok(Args {
            lua: positional.arg::<&custom::Lua>(0)?,
            code: positional.arg::<&str>(1)?,
        })
    }

    fn arg_signatures() -> Vec<ArgSignature> {
        vec![
            ArgSignature::new(
                "lua",
                "lua instance",
                <&custom::Lua as FromValue>::syntax_shape(),
            ),
            ArgSignature::new(
                "code",
                "lua code to evaluate",
                <&str as FromValue>::syntax_shape(),
            ),
        ]
    }
}

pub fn eval() -> Box<dyn PluginCommand<Plugin = crate::Plugin>> {
    Command::new::<_, Args>(
        "lua eval",
        "evaluate lua to a nushell value",
        |plugin, _engine, Args { lua, code }| plugin.eval_lua(lua, code),
    )
    .into()
}
