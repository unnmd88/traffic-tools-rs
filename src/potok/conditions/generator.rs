//! Генератор DDR-строки из AST
//!
//! Превращает выражение обратно в формат DDR, который ожидает пользователь.

use crate::potok::conditions::ast::*;

/// Опции генерации
#[derive(Debug, Clone)]
pub struct GenerateOptions {
    /// Префикс для каждого DDR: по умолчанию "ddr(D"
    pub prefix: String,
    
    /// Суффикс после числа: по умолчанию ")"
    pub suffix: String,
    
    /// Разделитель между элементами: по умолчанию " "
    pub separator: String,
    
    /// Использовать слова (and/or) или символы (&/|)
    pub use_symbols: bool,
}

impl Default for GenerateOptions {
    fn default() -> Self {
        Self {
            prefix: "ddr(D".to_string(),
            suffix: ")".to_string(),
            separator: " ".to_string(),
            use_symbols: false,
        }
    }
}

/// Основная функция генерации
pub fn to_condition_string(expr: &Expr) -> String {
    to_condition_string_with_options(expr, &GenerateOptions::default())
}

/// Генерация с опциями
pub fn to_condition_string_with_options(expr: &Expr, options: &GenerateOptions) -> String {
    match expr {
        Expr::Range(range) => generate_range(range, options),
        Expr::Binary { op, left, right } => {
            format!(
                "({}) {} ({})",
                to_condition_string_with_options(left, options),
                match (op, options.use_symbols) {
                    (BinaryOp::And, false) => "and",
                    (BinaryOp::And, true) => "&",
                    (BinaryOp::Or, false) => "or",
                    (BinaryOp::Or, true) => "|",
                },
                to_condition_string_with_options(right, options)
            )
        },
        Expr::WithMr { ddr_expr, mr } => {
            format!(
                "({}) and mr(G{})",
                to_condition_string_with_options(ddr_expr, options),
                mr
            )       
        },
    }
}

/// Генерация строки для диапазона
fn generate_range(range: &Range, options: &GenerateOptions) -> String {
    let numbers: Vec<String> = (range.start..=range.end)
        .map(|n| format!("{}{}{}", options.prefix, n, options.suffix))
        .collect();
    
    let operator_str = match (&range.operator, options.use_symbols) {
        (RangeOp::And, false) => "and",
        (RangeOp::And, true) => "&",
        (RangeOp::Or, false) => "or",
        (RangeOp::Or, true) => "|",
    };
    
    numbers.join(&format!(" {} ", operator_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_range() {
        let range = Range::new(1, 3, RangeOp::Or);
        assert_eq!(
            generate_range(&range, &GenerateOptions::default()),
            "ddr(D1) or ddr(D2) or ddr(D3)"
        );
    }

    #[test]
    fn test_generate_range_and() {
        let range = Range::new(4, 6, RangeOp::And);
        assert_eq!(
            generate_range(&range, &GenerateOptions::default()),
            "ddr(D4) and ddr(D5) and ddr(D6)"
        );
    }

    #[test]
    fn test_generate_binary() {
        let left = Expr::Range(Range::new(1, 3, RangeOp::Or));
        let right = Expr::Range(Range::new(4, 6, RangeOp::Or));
        let expr = Expr::Binary {
            op: BinaryOp::And,
            left: Box::new(left),
            right: Box::new(right),
        };

        assert_eq!(
            to_condition_string(&expr),
            "(ddr(D1) or ddr(D2) or ddr(D3)) and (ddr(D4) or ddr(D5) or ddr(D6))"
        );
    }

    #[test]
    fn test_with_symbols() {
        let range = Range::new(1, 3, RangeOp::Or);
        let options = GenerateOptions {
            use_symbols: true,
            ..Default::default()
        };
        
        assert_eq!(
            generate_range(&range, &options),
            "ddr(D1) | ddr(D2) | ddr(D3)"
        );
    }

    #[test]
    fn test_custom_prefix() {
        let range = Range::new(1, 3, RangeOp::Or);
        let options = GenerateOptions {
            prefix: "CH".to_string(),
            suffix: "".to_string(),
            ..Default::default()
        };
        
        assert_eq!(
            generate_range(&range, &options),
            "CH1 or CH2 or CH3"
        );
    }

    #[test]
    fn test_binary_with_symbols() {
        let left = Expr::Range(Range::new(1, 3, RangeOp::Or));
        let right = Expr::Range(Range::new(4, 6, RangeOp::Or));
        let expr = Expr::Binary {
            op: BinaryOp::And,
            left: Box::new(left),
            right: Box::new(right),
        };
        
        let options = GenerateOptions {
            use_symbols: true,
            ..Default::default()
        };

        assert_eq!(
            to_condition_string_with_options(&expr, &options),
            "(ddr(D1) | ddr(D2) | ddr(D3)) & (ddr(D4) | ddr(D5) | ddr(D6))"
        );
    }
}