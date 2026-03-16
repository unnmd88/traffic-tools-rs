use async_snmp::{ 
    Value,
    
};
use serde::Serialize;
use serde_json::json;
// use chrono::{DateTime, Utc};


pub enum AccessType {
    ReadOnly,
    ReadWrite,
}

#[derive(Debug, Clone, Copy)]
pub enum OidValueType {
    Integer,
    Gauge32,
    Unsigned32,
    Counter32,
    Counter64,
    OctetString,
    IpAddress,
    ObjectIdentifier,
    Timeticks,
    Null,
}



pub struct OidDefinition {
    pub name: &'static str,
    pub parser: fn(Value) -> Option<String>,
    pub value_type: OidValueType,
    pub access: AccessType,
}

#[derive(Debug, Serialize)]
pub struct OidResult {
    pub name: &'static str,
    pub oid: String,
    #[serde(skip)]
    pub raw_value: Value,
    pub raw_value_as_str: String,
    pub business_value: Option<String>,
    pub display: String,  // готовое форматированное сообщение
}

impl OidResult {
    pub fn new(
        name: &'static str, 
        oid: String, 
        raw: Value,
        parser: fn(Value) -> Option<String>,
    ) -> Self {
        let raw_as_str = raw.to_string();
        let parsed = parser(raw.clone());
        
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
            raw_value: raw,
            raw_value_as_str: raw_as_str,
            business_value: parsed,
            display,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SnmpResponse {
    pub host: String,
    pub host_id: u8,
    pub oids_request: Vec<String>,
    pub oids_response: Vec<OidResult>,
    pub timestamp: String,
}

impl SnmpResponse {
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
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
