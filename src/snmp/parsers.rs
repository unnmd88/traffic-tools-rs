use async_snmp::Value;

pub fn parse_val_as_str(value: Value) -> Option<String> {
    match value {
        Value::Integer(v) => Some(v.to_string()),
        Value::OctetString(v) => Some(String::from_utf8_lossy(&v).to_string()),
        Value::Counter32(v) => Some(v.to_string()),
        Value::Gauge32(v) => Some(v.to_string()),
        Value::Counter64(v) => Some(v.to_string()),
        // Для всех остальных типов возвращаем None
        _ => None,
    }
}