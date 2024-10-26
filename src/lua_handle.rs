use nu_protocol::{CustomValue, ShellError, Span, Value as NuValue};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuaHandle(Uuid);

impl LuaHandle {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

#[typetag::serde]
impl CustomValue for LuaHandle {
    fn clone_value(&self, span: Span) -> NuValue {
        NuValue::custom(Box::new(self.clone()), span)
    }

    fn type_name(&self) -> String {
        "LuaHandle".into()
    }

    fn to_base_value(&self, span: Span) -> Result<NuValue, ShellError> {
        Ok(NuValue::string("<LuaState>", span))
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
