use nu_plugin::SimplePluginCommand;
use nu_protocol::{LabeledError, Signature, SyntaxShape};

use crate::extensions::EvaluatedCallExt;
use crate::{custom, NuValue};

pub struct Eval;

impl SimplePluginCommand for Eval {
    type Plugin = crate::Plugin;

    fn name(&self) -> &str {
        "lua eval"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::new("lua eval")
            .add_help()
            .required("lua", SyntaxShape::Any, "the lua state")
            .required("code", SyntaxShape::String, "the lua code to evaluate")
    }

    fn description(&self) -> &str {
        "evaluate lua to a nushell value"
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        _input: &NuValue,
    ) -> Result<NuValue, LabeledError> {
        let lua = call.arg::<custom::Lua>(0)?;
        let code = call.arg::<str>(1)?;
        let value = plugin.eval_lua(lua, code)?;

        Ok(value)
    }
}
