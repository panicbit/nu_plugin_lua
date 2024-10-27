use extension_traits::extension;
use nu_plugin::EvaluatedCall;
use nu_protocol::{ShellError, Span, SyntaxShape};

use crate::custom::{self, PluginValue};
use crate::utils::{simple_error, type_error};
use crate::NuValue;

#[extension(pub trait NuValueExt)]
impl NuValue {
    fn as_specific_plugin_value(
        &self,
        type_name: impl AsRef<str>,
    ) -> Result<&PluginValue, ShellError> {
        self.as_custom_value()?
            .as_any()
            .downcast_ref::<PluginValue>()
            .ok_or_type_error(type_name, self.span())
    }

    fn as_plugin_value(&self) -> Result<&PluginValue, ShellError> {
        self.as_specific_plugin_value(PluginValue::TYPE_NAME)
    }

    fn as_lua(&self) -> Result<&custom::Lua, ShellError> {
        self.as_specific_plugin_value(custom::Lua::TYPE_NAME)?
            .as_lua(self.span())
    }
}

#[extension(pub trait OptionExt)]
impl<T> Option<T> {
    fn ok_or_type_error(self, type_name: impl AsRef<str>, span: Span) -> Result<T, ShellError> {
        self.ok_or_else(|| type_error(type_name, span))
    }

    fn ok_or_else_bug<F, R>(self, msg_fn: F) -> Result<T, ShellError>
    where
        F: FnOnce() -> R,
        R: AsRef<str>,
    {
        let msg = msg_fn();
        let msg = msg.as_ref();

        self.ok_or_else(|| simple_error(format!("BUG: {msg}")))
    }
}

#[extension(pub trait EvaluatedCallExt)]
impl EvaluatedCall {
    fn arg<T: FromArg + ?Sized>(&self, index: usize) -> Result<T::Output<'_>, ShellError> {
        self.positional.arg::<T>(index)
    }
}

#[extension(pub trait ArgSliceExt)]
impl [NuValue] {
    fn arg<T: FromArg + ?Sized>(&self, index: usize) -> Result<T::Output<'_>, ShellError> {
        let value = self
            .get(index)
            .ok_or_else_bug(|| format!("expected arg {index}"))?;

        T::from_arg(value)
    }
}

pub trait FromArg {
    type Output<'a>;

    fn from_arg(value: &NuValue) -> Result<Self::Output<'_>, ShellError>;
    fn syntax_shape() -> SyntaxShape;
}

impl FromArg for &'_ str {
    type Output<'a> = &'a str;

    fn from_arg(value: &NuValue) -> Result<Self::Output<'_>, ShellError> {
        value.as_str()
    }

    fn syntax_shape() -> SyntaxShape {
        SyntaxShape::String
    }
}
