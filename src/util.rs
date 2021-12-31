use nalgebra::Vector3;

pub type Vector = Vector3<f64>;
pub type Color = Vector3<f64>;
pub type Point = Vector3<f64>;

macro_rules! ternary {
    ($condition:expr, $t:expr, $f:expr) => {
        if $condition {
            $t
        } else {
            $f
        }
    };
}
