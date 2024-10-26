use nu_plugin::SimplePluginCommand;
use nu_protocol::{LabeledError, ShellError, Signature, Span, SyntaxShape, Value as NuValue};

use crate::lua_handle::LuaHandle;
use crate::Plugin;

pub(crate) struct LuaNew;

impl SimplePluginCommand for LuaNew {
    type Plugin = Plugin;

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

pub(crate) struct LuaEval;

impl SimplePluginCommand for LuaEval {
    type Plugin = Plugin;

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
        let lua_handle = call.nth(0).expect("BUG: arg 0 missing");
        let lua_handle = lua_handle
            .as_custom_value()?
            .as_any()
            .downcast_ref::<LuaHandle>()
            .ok_or_else(|| ShellError::TypeMismatch {
                err_message: "expected lua state".into(),
                span: lua_handle.span(),
            })?;
        let lua_code = call.nth(1).expect("BUG: arg 1 missing");
        let lua_code = lua_code.as_str()?;

        let value = plugin.eval_lua(lua_handle, lua_code)?;

        Ok(value)
    }
}
