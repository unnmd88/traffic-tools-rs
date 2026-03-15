use async_snmp::Value;

pub fn parse_val_as_str(value: Value) -> Option<String> {
    value.as_str().map(|s| s.to_string())
}