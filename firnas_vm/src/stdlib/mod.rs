use crate::value;

pub(crate) mod collection;
pub(crate) mod debug;
pub mod io;
pub(crate) mod math;
pub(crate) mod time;

pub struct StdFunc {
    pub name: String,
    pub func: value::Value,
}
