use std::sync::Arc;

use fnv::FnvHashMap;
use mlua::Lua;
use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_protocol::{LabeledError, Record, ShellError, Span};
use parking_lot::{Mutex, RwLock};
use uuid::Uuid;

mod command;
mod custom;
mod extensions;
mod utils;

type SharedLua = Arc<Mutex<Lua>>;
type NuValue = nu_protocol::Value;
type LuaValue<'lua> = mlua::Value<'lua>;

fn main() {
    serve_plugin(&Plugin::new(), MsgPackSerializer);
}

pub struct Plugin {
    states: RwLock<FnvHashMap<Uuid, Arc<Mutex<Lua>>>>,
}

impl Plugin {
    fn new() -> Self {
        Self {
            states: RwLock::new(FnvHashMap::default()),
        }
    }

    fn create_lua(&self) -> (custom::Lua, SharedLua) {
        let uuid = Uuid::new_v4();
        let lua = Arc::new(Mutex::new(Lua::new()));
        let custom = custom::Lua::new(uuid);

        self.states.write().insert(uuid, lua.clone());

        (custom, lua)
    }

    fn destroy_lua(&self, lua: &custom::Lua) {
        self.states.write().remove(&lua.uuid());
    }

    fn get_lua(&self, lua: &custom::Lua) -> Option<Arc<Mutex<Lua>>> {
        let states = self.states.read();
        let lua = states.get(&lua.uuid())?;

        Some(Arc::clone(lua))
    }

    fn eval_lua(&self, lua: &custom::Lua, lua_code: &str) -> Result<NuValue, ShellError> {
        let Some(lua) = self.get_lua(lua) else {
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
        _engine: &nu_plugin::EngineInterface,
        custom_value: Box<dyn nu_protocol::CustomValue>,
    ) -> Result<(), nu_protocol::LabeledError> {
        if let Some(value) = custom_value.as_any().downcast_ref::<custom::PluginValue>() {
            match value {
                custom::PluginValue::Lua(lua) => self.destroy_lua(lua),
            }
        }

        Ok(())
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![Box::new(command::New), Box::new(command::Eval)]
    }
}

fn lua_to_nushell(value: LuaValue) -> Result<NuValue, ShellError> {
    let span = Span::unknown();

    Ok(match value {
        LuaValue::Nil => NuValue::nothing(span),
        LuaValue::Boolean(v) => NuValue::bool(v, span),
        LuaValue::Integer(v) => NuValue::int(v, span),
        LuaValue::Number(v) => NuValue::float(v, span),
        LuaValue::String(v) => v
            .to_str()
            .map(|v| NuValue::string(v.to_string(), span))
            .unwrap_or_else(|_| NuValue::binary(v.as_bytes().to_vec(), span)),
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
        }
        LuaValue::Function(function) => {
            let function = function.to_pointer();
            let function = format!("function: {function:p}");

            NuValue::string(function, span)
        }
        LuaValue::Thread(thread) => {
            let thread = thread.to_pointer();
            let thread = format!("thread: {thread:p}");

            NuValue::string(thread, span)
        }
        LuaValue::LightUserData(light_user_data) => {
            let light_user_data = light_user_data.0;
            let light_user_data = format!("light_user_data: {light_user_data:p}");

            NuValue::string(light_user_data, span)
        }
        LuaValue::UserData(user_data) => {
            let user_data = user_data.to_pointer();
            let user_data = format!("user_data: {user_data:p}");

            NuValue::string(user_data, span)
        }
        LuaValue::Error(_error) => {
            return Err(LabeledError::new(
                "converting mlua internal errors to nushell is not supported",
            )
            .into());
        }
    })
}
