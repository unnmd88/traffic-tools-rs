
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    ReadOnly,
    ReadWrite,
    WriteOnly,
}

impl Access {
    pub fn as_str(&self) -> &'static str {
        match self {
            Access::ReadOnly => "read-only",
            AcceAccessssType::ReadWrite => "read-write",
        }
    }
}

#[derive(Debug, Clone)]
pub struct OidDef {
    /// Имя параметра из документации (например "SwarcoUTCTrafftechPhaseStatus")
    pub name: &'static str,
    /// Числовой OID (например [1.3.6.1.4.1.1618.3.7.2.11.2])
    pub oid: &'static [u32],
    /// Тип доступа (read-only / read-write)
    pub access: AccessType,
    /// Описание параметра
    pub description: &'static str,
}


impl OidDef {
    /// Преобразовать OID в строку "1.3.6.1.4.1.1618.3.7.2.11.2"
    pub fn to_string(&self) -> String {
        self.oid
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(".")
    }
    
    /// Получить OID как срез (для async_snmp)
    pub fn as_slice(&self) -> &[u32] {
        self.oid
    }

    pub fn is_writable(&self) -> bool {
        self.access == AccessType::ReadWrite
    }

}
