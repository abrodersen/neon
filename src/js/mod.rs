//! Types and traits representing JavaScript values.

pub mod binary;
pub mod error;
pub mod class;
pub mod async;

pub use internal::js::{Value, Variant, Object, Key, JsValue, JsUndefined, JsNull, JsBoolean, JsInteger, JsNumber, JsString, JsObject, JsArray, JsFunction};
