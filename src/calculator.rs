// src/calculator.rs

use crate::operations::{add, divide, multiply, subtract};

pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn calculate(a: f64, b: f64, operation: Operation) -> f64 {
    match operation {
        Operation::Add => add(a, b),
        Operation::Subtract => subtract(a, b),
        Operation::Multiply => multiply(a, b),
        Operation::Divide => divide(a, b),
    }
}
