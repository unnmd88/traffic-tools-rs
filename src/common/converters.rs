// traffic-utils/src/converters.rs

//! Общие функции для преобразования данных
//!
//! Содержит только действительно общие конвертеры,
//! не привязанные к предметной области.

/// Преобразовать строку в ASCII-коды (вектор чисел)
/// 
/// # Пример
/// ```
/// let codes = to_ascii_codes("ABC");
/// assert_eq!(codes, vec![65, 66, 67]);
/// ```
pub fn to_ascii_codes(s: &str) -> Vec<u8> {
    s.trim().bytes().collect()
}

/// Преобразовать строку в ASCII-коды, объединённые разделителем
/// 
/// # Пример
/// ```
/// use traffic_utils::converters::to_ascii_delimited;
/// 
/// let result = to_ascii_delimited("ABC", ".");
/// assert_eq!(result, "65.66.67");
/// ```
pub fn to_ascii_delimited(s: &str, delimiter: &str) -> String {
    s.trim()
        .bytes()
        .map(|b| b.to_string())
        .collect::<Vec<_>>()
        .join(delimiter)
}


/// Преобразует строку в SCN-формат для дорожных контроллеров
/// 
/// SCN-формат используется в различных протоколах для представления
/// строковых значений в виде ASCII-кодов с префиксом.
/// 
/// # Формат
/// `.1.{длина}.{ascii_коды_через_точку}`
/// 
/// # Аргументы
/// * `s` - входная строка (пробелы в начале и конце будут обрезаны)
/// 
/// # Пример
/// ```
/// use traffic_controller::format::scn::to_scn_format;
/// 
/// let result = to_scn_format("CO4554");
/// assert_eq!(result, ".1.6.67.79.52.53.53.52");
/// 
/// let result = to_scn_format("  ABC  ");
/// assert_eq!(result, ".1.3.65.66.67");
/// ```
pub fn to_scn_format(s: &str) -> String {
    let trimmed = s.trim();
    let len = trimmed.len();
    let codes_str = to_ascii_delimited(trimmed, ".");
    format!(".1.{}.{}", len, codes_str)
}


/// Преобразовать строку в hex-представление
/// 
/// # Пример
/// ```
/// use traffic_utils::converters::to_hex;
/// 
/// let result = to_hex("ABC");
/// assert_eq!(result, "414243");
/// ```
pub fn to_hex(s: &str) -> String {
    s.trim()
        .bytes()
        .map(|b| format!("{:02X}", b))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_ascii_codes() {
        assert_eq!(to_ascii_codes("ABC"), vec![65, 66, 67]);
    }

    #[test]
    fn test_to_ascii_delimited() {
        assert_eq!(to_ascii_delimited("ABC", "."), "65.66.67");
        assert_eq!(to_ascii_delimited("ABC", "-"), "65-66-67");
    }

    #[test]
    fn test_to_hex() {
        assert_eq!(to_hex("ABC"), "414243");
        assert_eq!(to_hex("123"), "313233");
    }
}