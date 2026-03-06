//! Модуль для работы с DDR-выражениями
//!
//! Этот модуль позволяет парсить строки вида "1-3", "or 1-3", 
//! "(or 1-3) and (or 4-6)" и превращать их в структуры данных,
//! а также генерировать обратно строки в формате DDR.
//!
//! # Пример
//! ```
//! let expr = parse_input_expression("(or 1-3) and (or 4-6)").unwrap();
//! let result = to_condition_string(&expr);
//! println!("{}", result); // (ddr(D1) or ddr(D2) or ddr(D3)) and (ddr(D4) or ddr(D5) or ddr(D6))
//! ```
//! 


// Объявляем подмодули (файлы в той же папке)
mod ast;        // ast.rs — структуры данных
mod parser;     // parser.rs — разбор строки в AST
mod generator;  // generator.rs — преобразование AST в строку
mod error;      // error.rs — типы ошибок

// Реэкспортируем самое важное наружу
// Теперь пользователь сможет писать:
// use ddr_conditions::{parse_ddr_expression, Expr, Range, ParseError};
pub use ast::{Expr, Range, RangeOp, BinaryOp};
pub use parser::parse_input_expression;
pub use generator::{to_condition_string, GenerateOptions};
pub use error::ParseError;