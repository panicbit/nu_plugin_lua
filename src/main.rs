use std::collections::HashMap;
use std::sync::Arc;

use lua_handle::LuaHandle;
use mlua::{Lua, Value as LuaValue};
use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_protocol::{LabeledError, Record, ShellError, Span, Value as NuValue};
use parking_lot::{Mutex, RwLock};
use uuid::Uuid;

mod command;
mod lua_handle;

fn main() {
    serve_plugin(&Plugin::new(), MsgPackSerializer);
}

struct Plugin {
    // TODO: use fnv
    states: RwLock<HashMap<Uuid, Arc<Mutex<Lua>>>>,
}

impl Plugin {
    fn new() -> Self {
        Self {
            states: RwLock::new(HashMap::new()),
        }
    }

    fn create_lua(&self) -> LuaHandle {
        let uuid = Uuid::new_v4();
        let lua = Arc::new(Mutex::new(Lua::new()));

        self.states.write().insert(uuid, lua);

        LuaHandle::new(uuid)
    }

    fn destroy_lua(&self, lua_handle: &LuaHandle) {
        self.states.write().remove(&lua_handle.uuid());
    }

    fn get_lua(&self, lua_handle: &LuaHandle) -> Option<Arc<Mutex<Lua>>> {
        let states = self.states.read();
        let lua = states.get(&lua_handle.uuid())?;

        Some(Arc::clone(lua))
    }

    fn eval_lua(&self, lua_handle: &LuaHandle, lua_code: &str) -> Result<NuValue, ShellError> {
        let Some(lua) = self.get_lua(lua_handle) else {
            return Err(LabeledError::new("lua handle is invalid").into());
        };

        let lua = lua.lock();
        let value = lua
            .load(lua_code)
            .eval::<LuaValue>()
            .map_err(|err| LabeledError::new(err.to_string()))?;

        lua_to_nushell(value)
    }
}

impl nu_plugin::Plugin for Plugin {
    fn custom_value_dropped(
        &self,
        engine: &nu_plugin::EngineInterface,
        custom_value: Box<dyn nu_protocol::CustomValue>,
    ) -> Result<(), nu_protocol::LabeledError> {
        if let Some(lua_handle) = custom_value.as_any().downcast_ref::<LuaHandle>() {
            self.destroy_lua(lua_handle);
        }

        Ok(())
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(command::LuaNew),
            Box::new(command::LuaEval),
        ]
    }
}

fn lua_to_nushell(value: LuaValue) -> Result<NuValue, ShellError> {
    let span = Span::unknown();

    Ok(match value {
        LuaValue::Nil => NuValue::nothing(span),
        LuaValue::Boolean(v) => NuValue::bool(v, span),
        LuaValue::Integer(v) => NuValue::int(v, span),
        LuaValue::Number(v) => NuValue::float(v, span),
        LuaValue::String(v) => {
            v.to_str()
            .map(|v| NuValue::string(v.to_string(), span))
            .unwrap_or_else(|_| NuValue::binary(v.as_bytes().to_vec(), span))
        },
        LuaValue::Table(table) => {
            let mut records = Vec::new();

            for pair in table.pairs::<LuaValue, LuaValue>() {
                let (k, v) = pair.map_err(|err| LabeledError::new(err.to_string()))?;
                let k = lua_to_nushell(k)?;
                let v = lua_to_nushell(v)?;

                let mut record = Record::new();
                record.push("index", k);
                record.push("item", v);

                let record = NuValue::record(record, span);
                records.push(record);
            }

            NuValue::list(records, span)
        },
        LuaValue::Function(function) => {
            let function = function.to_pointer();
            let function = format!("function: {function:p}");

            NuValue::string(function, span)
        },
        LuaValue::Thread(thread) => todo!(),
        LuaValue::LightUserData(light_user_data) => todo!(),
        LuaValue::UserData(any_user_data) => todo!(),
        LuaValue::Error(error) => todo!(),
    })
}
