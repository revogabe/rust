// src/operations.rs

pub fn add(a: f64, b: f64) -> f64 {
   return a + b
}

pub fn subtract(a: f64, b: f64) -> f64 {
   return a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
   return a * b
}

pub fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("Não pode dividir por zero!");
    }
    return a / b
}
