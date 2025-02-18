use nu_plugin_helpers::FromValue;
use nu_protocol::{ShellError, Span, SyntaxShape};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::extensions::NuValueExt;
use crate::NuValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lua(Uuid);

impl Lua {
    pub const TYPE_NAME: &str = "lua instance";

    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn uuid(&self) -> Uuid {
        self.0
    }

    pub fn to_base_value(&self, span: Span) -> Result<NuValue, ShellError> {
        Ok(NuValue::string("<Lua>", span))
    }
}

impl FromValue for &Lua {
    type Output<'a> = &'a Lua;

    fn from_value(value: &NuValue) -> Result<Self::Output<'_>, ShellError> {
        value.as_lua()
    }

    fn syntax_shape() -> SyntaxShape {
        SyntaxShape::Any
    }
}
