use serde::{Deserialize, Serialize};

pub mod pagination;

/// This macro allows you to use a ternary operator in Rust.
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

/// A simple struct that represents a field in a document.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Field {
    pub field: String,
    pub value: String,
}
