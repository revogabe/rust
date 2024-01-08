// src/main.rs

mod operations;
mod calculator;
use calculator::{calculate, Operation};
use text_io::read;

fn main() {
    println!("Escolha a operação (Digite o número):");
    println!("1: Adição");
    println!("2: Subtração");
    println!("3: Multiplicação");
    println!("4: Divisão");

    let operacao: i32 = read!();

    let op = match operacao {
        1 => Operation::Add,
        2 => Operation::Subtract,
        3 => Operation::Multiply,
        4 => Operation::Divide,
        _ => {
            println!("Operação inválida!");
            return;
        }
    };

    println!("Digite o primeiro número:");
    let a: f64 = read!();

    println!("Digite o segundo número:");
    let b: f64 = read!();

    let result = calculate(a, b, op);
    println!("O resultado é: {}", result);
}
