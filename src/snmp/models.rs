use async_snmp::{ 
    Value,
};
use serde::Serialize;
// use chrono::{DateTime, Utc};


pub struct OidDefinition {
    name: &'static str,
    oid: Vec<u16>,
    parser: fn(Value) -> String,
}

#[derive(Debug, Serialize)]
pub struct OidResult {
    pub name: &'static str,
    pub oid: &'static str,
    #[serde(skip)]
    pub raw: Value,
    pub raw_as_str: String,
    pub parsed: Option<String>,
    pub display: String,  // готовое форматированное сообщение
}

impl OidResult {
    pub fn new(
        name: &'static str, 
        oid: &'static str, 
        raw: Value,
        parser: fn(&Value) -> Option<String>,
    ) -> Self {
        let raw_as_str = raw.to_string();
        let parsed = parser(&raw);
        
        // Форматируем parsed красиво
        let parsed_display = match &parsed {
            Some(v) => v.as_str(),
            None => "None",
        };
        
        let display = format!(
            "Oid={}[{}] значение={} преобразованное={}",
            name, 
            oid,
            raw.to_string(), 
            parsed_display
        );
        
        Self {
            name,
            oid,
            raw,
            raw_as_str,
            parsed,
            display,
        }
    }
}


// #[derive(Debug, Clone, Serialize)]
// pub struct OidResult {
//     #[serde(skip)]
//     pub raw: Value,           // сырое значение (не в JSON)
//     pub value: String,        // значение как строка (всегда в JSON)
//     pub name: String,
//     pub oid: String,
//     // pub timestamp: DateTime<Utc>,
// }

// impl OidResult {
//     pub fn new(raw: Value, name: String, oid: String) -> Self {
//         let value = raw.to_string(); // Value умеет сам себя отображать
        
//         Self {
//             raw,
//             value,
//             name,
//             oid,
//             // timestamp: Utc::now(),
//         }
//     }
// }
