// Пример 1: Простой диапазон
// "1-3" → range_parser → Expr::Range

// Пример 2: Диапазон с оператором
// "and 1-3" → range_parser → Expr::Range(Range { op: And, ... })

// Пример 3: Скобки с диапазоном
// "(1-3)" → parens_parser → expr → range_parser → Expr::Range

// Пример 4: AND двух диапазонов
// "(1-3) and (4-6)" 
// parens_parser → Expr::Range
// binary_op_parser(And)
// parens_parser → Expr::Range
// → Expr::Binary { op: And, left: Range(1-3), right: Range(4-6) }

// Пример 5: Цепочка с приоритетом
// "(1-3) and (4-6) or (7-9)"
// precedence расставит приоритеты:
// → Expr::Binary { 
//     op: Or,
//     left: Expr::Binary { op: And, left: Range(1-3), right: Range(4-6) },
//     right: Range(7-9)
//   }

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::{map, opt, value},
    sequence::delimited,
    Parser,
    error::Error,
};

use super::ast::*;
use super::error::ParseError;

/// Основная функция для внешнего использования
pub fn parse_input_expression(input: &str) -> Result<Expr, ParseError> {
    let input = input.trim();
    
    if input.is_empty() {
        return Err(ParseError::MissingOperand("выражение".to_string()));
    }

    // Одинокий оператор
    if input == "&" || input == "|" || input == "and" || input == "or" {
        return Err(ParseError::MissingOperand(input.to_string()));
    }

    if input.ends_with('-') && input.chars().filter(|c| *c == '-').count() == 1 {
        return Err(ParseError::InvalidRange);
    }
    
    // Предварительные проверки
    if let Some(op) = check_trailing_operator(input) {
        return Err(ParseError::MissingOperand(op));
    }
    
    check_paren_balance(input)?;
    check_empty_parens(input)?;
    
    // Используем новый парсер с поддержкой MR
    match expr_with_mr_parser().parse(input) {
        Ok(("", expr)) => {
            // Всё выражение успешно распарсилось
            if let Expr::Range(range) = &expr {
                if range.start > range.end {
                    return Err(ParseError::RangeStartGreaterThanEnd(range.start, range.end));
                }
            }
            Ok(expr)
        }
        Ok((remaining, _)) => {
            // Распарсилось, но остались символы
            let remaining = remaining.trim();
            if remaining.is_empty() {
                unreachable!() // этого не должно случиться
            } else if remaining.starts_with(')') {
                Err(ParseError::ExtraClosingParen)
            } else {
                Err(ParseError::ExtraInput(remaining.to_string()))
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            // Ошибка парсинга
            let input_preview = if input.len() > 30 {
                format!("{}...", &input[..30])
            } else {
                input.to_string()
            };
            
            match e.code {
                nom::error::ErrorKind::Digit => {
                    Err(ParseError::ExpectedNumber(input_preview))
                }
                nom::error::ErrorKind::Char => {
                    if !input.contains('-') {
                        Err(ParseError::InvalidRange)
                    } else {
                        let first_char = input.chars().next().unwrap_or('?');
                        Err(ParseError::UnexpectedChar(first_char, 0))
                    }
                }
                nom::error::ErrorKind::Tag => {
                    if input.starts_with('(') && !input.contains(')') {
                        Err(ParseError::UnclosedParen)
                    } else {
                        Err(ParseError::UnknownOperator(input_preview))
                    }
                }
                _ => {
                    eprintln!("[DEBUG] Ошибка парсинга: {:?} для input: {}", e, input);
                    Err(ParseError::InternalError(format!("{:?}", e)))
                }
            }
        }
        Err(e) => {
            eprintln!("[DEBUG] Неожиданная ошибка: {:?}", e);
            Err(ParseError::InternalError(format!("{:?}", e)))
        }
    }
}

/// Проверка на висящий оператор в конце
fn check_trailing_operator(input: &str) -> Option<String> {
    let trimmed = input.trim_end(); // убираем пробелы только справа
    
    // Проверяем все варианты операторов (с пробелом и без)
    let patterns = [
        ("and", "and"), ("and ", "and"),
        ("or", "or"), ("or ", "or"),
        ("&", "&"), ("& ", "&"),
        ("|", "|"), ("| ", "|"),
    ];
    
    for (pattern, op) in patterns {
        if trimmed.ends_with(pattern) {
            // Проверяем, что перед оператором есть валидное выражение
            let before_op = &trimmed[..trimmed.len() - pattern.len()].trim();
            
            // Если перед оператором что-то есть и это не пустота
            if !before_op.is_empty() {
                // Проверяем последний символ
                if let Some(last_char) = before_op.chars().last() {
                    // Если последний символ - цифра или скобка, значит оператор висящий
                    if last_char.is_digit(10) || last_char == ')' {
                        return Some(op.to_string());
                    }
                }
            }
        }
    }
    None
}

/// Проверка баланса скобок
fn check_paren_balance(input: &str) -> Result<(), ParseError> {
    let mut balance = 0;
    
    for c in input.chars() {
        match c {
            '(' => balance += 1,
            ')' => {
                if balance == 0 {
                    return Err(ParseError::ExtraClosingParen);
                }
                balance -= 1;
            }
            _ => {}
        }
    }
    
    if balance > 0 {
        Err(ParseError::UnclosedParen)
    } else {
        Ok(())
    }
}

/// Проверка на пустые скобки
fn check_empty_parens(input: &str) -> Result<(), ParseError> {
    if input.contains("()") {
        return Err(ParseError::EmptyParens);
    }
    Ok(())
}

/// Парсер числа
fn number_parser<'a>() -> impl Parser<&'a str, Output = u32, Error = Error<&'a str>> {
    map(digit1, |s: &str| s.parse().unwrap())
}

/// Обёртка для игнорирования пробелов
fn ws<'a, Par>(parser: Par) -> impl Parser<&'a str, Output = Par::Output, Error = Error<&'a str>>
where
    Par: Parser<&'a str, Error = Error<&'a str>>,
{
    delimited(multispace0, parser, multispace0)
}

/// Парсер оператора внутри диапазона (or/and/|/&) - опционально
fn range_op_parser<'a>() -> impl Parser<&'a str, Output = Option<RangeOp>, Error = Error<&'a str>> {
    opt(ws(alt((
        value(RangeOp::And, alt((tag("and"), tag("&")))),
        value(RangeOp::Or, alt((tag("or"), tag("|")))),
    ))))
}

/// Парсер диапазона: [or/and] число-число
fn range_parser<'a>() -> impl Parser<&'a str, Output = Range, Error = Error<&'a str>> {
    move |input: &'a str| {
        let (input, op) = range_op_parser().parse(input)?;
        let (input, start) = ws(number_parser()).parse(input)?;
        let (input, _) = ws(char('-')).parse(input)?;
        let (input, end) = ws(number_parser()).parse(input)?;
        
        // Конвертируем u32 в u16
        let start_u16 = start as u16;
        let end_u16 = end as u16;
        
        Ok((input, Range::new(start_u16, end_u16, op.unwrap_or(RangeOp::Or))))
    }
}

/// Парсер выражения в скобках
fn parens_parser<'a>() -> impl Parser<&'a str, Output = Expr, Error = Error<&'a str>> {
    move |input: &'a str| {
        let (input, _) = ws(char('(')).parse(input)?;
        let (input, expr) = expr_parser().parse(input)?;
        let (input, _) = ws(char(')')).parse(input)?;
        
        Ok((input, expr))
    }
}

/// Парсер бинарного оператора (and/or/&/|)
fn binary_op_parser<'a>() -> impl Parser<&'a str, Output = BinaryOp, Error = Error<&'a str>> {
    ws(alt((
        value(BinaryOp::And, alt((tag("and"), tag("&")))),
        value(BinaryOp::Or, alt((tag("or"), tag("|")))),
    )))
}

/// Парсер атомарного выражения (скобки или диапазон)
fn atom_parser<'a>() -> impl Parser<&'a str, Output = Expr, Error = Error<&'a str>> {
    alt((
        parens_parser(),
        map(range_parser(), Expr::Range),
    ))
}

/// Парсер выражения (без MR)
fn expr_parser<'a>() -> impl Parser<&'a str, Output = Expr, Error = Error<&'a str>> {
    move |mut input: &'a str| {
        let (rest, mut left) = atom_parser().parse(input)?;
        input = rest;
        
        loop {
            match binary_op_parser().parse(input) {
                Ok((rest, op)) => {
                    let (rest, right) = atom_parser().parse(rest)?;
                    left = Expr::Binary {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                    input = rest;
                }
                Err(nom::Err::Error(_)) => break,
                Err(e) => return Err(e),
            }
        }
        
        Ok((input, left))
    }
}

/// Парсер для MR: ", число"
fn mr_parser<'a>() -> impl Parser<&'a str, Output = u16, Error = Error<&'a str>> {
    move |input: &'a str| {
        let (input, _) = char(',')(input)?;  // просто запятая, без ws
        let (input, num) = ws(number_parser()).parse(input)?;  // число с пробелами
        let num_u16 = num as u16;
        
        Ok((input, num_u16))
    }
}

/// Парсер с поддержкой MR
fn expr_with_mr_parser<'a>() -> impl Parser<&'a str, Output = Expr, Error = Error<&'a str>> {
    move |input: &'a str| {
        // Сначала парсим DDR-выражение
        match expr_parser().parse(input) {
            Ok((rest, ddr_expr)) => {
                // Пробуем найти MR
                match mr_parser().parse(rest) {
                    Ok((final_rest, mr_num)) => {
                        // Нашли MR
                        let expr = Expr::WithMr {
                            ddr_expr: Box::new(ddr_expr),
                            mr: mr_num,
                        };
                        Ok((final_rest, expr))
                    }
                    Err(nom::Err::Error(_)) | Err(nom::Err::Failure(_)) => {
                        // MR нет, возвращаем просто DDR
                        Ok((rest, ddr_expr))
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_range() {
        let (_, range) = range_parser().parse("1-3").unwrap();
        assert_eq!(range.start, 1);
        assert_eq!(range.end, 3);
        assert_eq!(range.operator, RangeOp::Or);
        
        let (_, range) = range_parser().parse("or 1-3").unwrap();
        assert_eq!(range.operator, RangeOp::Or);
        
        let (_, range) = range_parser().parse("and 4-6").unwrap();
        assert_eq!(range.operator, RangeOp::And);
    }
    
    #[test]
    fn test_parens() {
        let (_, expr) = parens_parser().parse("(1-3)").unwrap();
        match expr {
            Expr::Range(range) => {
                assert_eq!(range.start, 1);
                assert_eq!(range.end, 3);
            }
            _ => panic!("Expected range"),
        }
    }
    
    #[test]
    fn test_binary() {
        let (_, expr) = expr_parser().parse("(1-3) and (4-6)").unwrap();
        match expr {
            Expr::Binary { op, left, right } => {
                assert_eq!(op, BinaryOp::And);
                match *left {
                    Expr::Range(range) => {
                        assert_eq!(range.start, 1);
                        assert_eq!(range.end, 3);
                    }
                    _ => panic!("Expected range"),
                }
                match *right {
                    Expr::Range(range) => {
                        assert_eq!(range.start, 4);
                        assert_eq!(range.end, 6);
                    }
                    _ => panic!("Expected range"),
                }
            }
            _ => panic!("Expected binary expression"),
        }
    }
    
    #[test]
    fn test_chain() {
        let (_, expr) = expr_parser().parse("(1-3) and (4-6) or (7-9)").unwrap();
        match expr {
            Expr::Binary { op, left, right } => {
                assert_eq!(op, BinaryOp::Or);
                match *left {
                    Expr::Binary { op: and_op, .. } => {
                        assert_eq!(and_op, BinaryOp::And);
                    }
                    _ => panic!("Expected AND as left operand"),
                }
                match *right {
                    Expr::Range(range) => {
                        assert_eq!(range.start, 7);
                        assert_eq!(range.end, 9);
                    }
                    _ => panic!("Expected range as right operand"),
                }
            }
            _ => panic!("Expected OR at top level"),
        }
    }

    #[test]
    fn test_missing_operand() {
        assert!(matches!(
            parse_input_expression("1-3 and"),
            Err(ParseError::MissingOperand(op)) if op == "and"
        ));
        
        assert!(matches!(
            parse_input_expression("(1-3) or"),
            Err(ParseError::MissingOperand(op)) if op == "or"
        ));
        
        assert!(matches!(
            parse_input_expression("|"),
            Err(ParseError::MissingOperand(op)) if op == "|"
        ));
    }
    

    #[test]
    fn test_unclosed_paren() {
        assert!(matches!(
            parse_input_expression("(1-3"),
            Err(ParseError::UnclosedParen)
        ));
        
        assert!(matches!(
            parse_input_expression("((1-3) and (4-6)"),
            Err(ParseError::UnclosedParen)
        ));
    }
    
    #[test]
    fn test_extra_closing_paren() {
        assert!(matches!(
            parse_input_expression("(1-3))"),
            Err(ParseError::ExtraClosingParen)
        ));
    }
    
    #[test]
    fn test_empty_parens() {
        assert!(matches!(
            parse_input_expression("()"),
            Err(ParseError::EmptyParens)
        ));
    }
    
    #[test]
    fn test_invalid_range() {
        let result = parse_input_expression("1-");
        println!("result = {:?}", result);
        
        assert!(matches!(
            result,
            Err(ParseError::InvalidRange)
        ));
    }
    
    #[test]
    fn test_range_start_greater_than_end() {
        assert!(matches!(
            parse_input_expression("5-3"),
            Err(ParseError::RangeStartGreaterThanEnd(5, 3))
        ));
    }

    // Новые тесты для MR
    #[test]
    fn test_mr_simple() {
        let expr = parse_input_expression("1-3, 12").unwrap();
        match expr {
            Expr::WithMr { ddr_expr, mr } => {
                assert_eq!(mr, 12);
                match *ddr_expr {
                    Expr::Range(range) => {
                        assert_eq!(range.start, 1);
                        assert_eq!(range.end, 3);
                    }
                    _ => panic!("Expected range"),
                }
            }
            _ => panic!("Expected WithMr"),
        }
    }

    #[test]
    fn test_mr_with_spaces() {
        let expr = parse_input_expression("1-3,12").unwrap();
        match expr {
            Expr::WithMr { ddr_expr, mr } => {
                assert_eq!(mr, 12);
                match *ddr_expr {
                    Expr::Range(range) => {
                        assert_eq!(range.start, 1);
                        assert_eq!(range.end, 3);
                    }
                    _ => panic!("Expected range"),
                }
            }
            _ => panic!("Expected WithMr"),
        }
    }

    #[test]
    fn test_complex_with_mr() {
        let expr = parse_input_expression("(1-3) and (7-8), 4").unwrap();
        match expr {
            Expr::WithMr { ddr_expr, mr } => {
                assert_eq!(mr, 4);
                match *ddr_expr {
                    Expr::Binary { op, left, right } => {
                        assert_eq!(op, BinaryOp::And);
                    }
                    _ => panic!("Expected binary expression"),
                }
            }
            _ => panic!("Expected WithMr"),
        }
    }
}