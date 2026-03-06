//! AST (Abstract Syntax Tree) для DDR-выражений
//!
//! Этот модуль содержит структуры данных, которые представляют
//! разобранное выражение пользователя.

/// Основное выражение.
///
/// Выражением может быть:
/// - Простой диапазон: "or 1-3", "and 4-6", "1-3" (OR по умолчанию)
/// - Комбинация выражений: "(or 1-3) and (or 4-6)"
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Диапазон DDR номеров (самый простой случай)
    Range(Range),
    
    /// Бинарная операция: левое выражение, оператор, правое выражение
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    WithMr { 
        ddr_expr: Box<Expr>, 
        mr: u16,
     },
}

/// Диапазон DDR номеров.
///
/// Хранит начальный и конечный номер, а также оператор внутри диапазона.
/// Пример: "and 1-3" → Range { start: 1, end: 3, operator: RangeOp::And }
#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub start: u16,
    pub end: u16,
    pub operator: RangeOp,
}

/// Оператор внутри диапазона.
///
/// Определяет, как соединяются элементы внутри одного диапазона:
/// - Or: ddr(D1) or ddr(D2) or ddr(D3)  (по умолчанию)
/// - And: ddr(D1) and ddr(D2) and ddr(D3)
#[derive(Debug, Clone, PartialEq)]
pub enum RangeOp {
    Or,  // значение по умолчанию, если оператор не указан
    And,
}

/// Оператор между выражениями.
///
/// Используется для соединения целых выражений:
/// - (or 1-3) and (or 4-6) → BinaryOp::And
/// - (or 1-3) or (and 4-6)  → BinaryOp::Or
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    And,
    Or,
}

/// Конструкторы для удобства создания структур.
impl Range {
    /// Создание нового диапазона
    ///
    /// # Пример
    /// ```
    /// use traffic_controller::potok::conditions::{Range, RangeOp};
    ///
    /// let range = Range::new(1, 3, RangeOp::Or);
    /// assert_eq!(range.start, 1);
    /// assert_eq!(range.end, 3);
    /// ```
    pub fn new(start: u16, end: u16, operator: RangeOp) -> Self {
        Self { start, end, operator }
    }
}