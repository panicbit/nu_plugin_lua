use nu_protocol::{CustomValue, ShellError, Span};
use serde::{Deserialize, Serialize};

use crate::utils::type_error;
use crate::{custom, NuValue};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum PluginValue {
    Lua(custom::Lua),
}

impl PluginValue {
    pub const TYPE_NAME: &str = "lua plugin value";

    pub fn as_lua(&self, span: Span) -> Result<&custom::Lua, ShellError> {
        match self {
            Self::Lua(lua) => Ok(lua),
            #[expect(unreachable_patterns)]
            _ => Err(type_error("lua instance", span)),
        }
    }
}

#[typetag::serde]
impl CustomValue for PluginValue {
    fn clone_value(&self, span: Span) -> NuValue {
        NuValue::custom(Box::new(self.clone()), span)
    }

    fn type_name(&self) -> String {
        match self {
            PluginValue::Lua(_) => "Lua",
        }
        .into()
    }

    fn to_base_value(&self, span: Span) -> Result<NuValue, ShellError> {
        match self {
            PluginValue::Lua(lua) => lua.to_base_value(span),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn notify_plugin_on_drop(&self) -> bool {
        true
    }
}
