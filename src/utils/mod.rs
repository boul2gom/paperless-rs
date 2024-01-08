use serde::{Deserialize, Serialize};

pub mod pagination;

#[macro_export]
macro_rules! ternary {
    ($condition: expr, $_true: expr, $_false: expr) => {
        if $condition {
            $_true
        } else {
            $_false
        }
    };
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Field {
    pub field: String,
    pub value: String,
}
