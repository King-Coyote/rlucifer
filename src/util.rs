use crate::basic_types::*;
use crate::objects::*;


pub fn float_zero(val: f32) -> bool {
    val.abs() < 0.0001
}