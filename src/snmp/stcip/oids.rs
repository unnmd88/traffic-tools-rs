use crate::snmp::models::{AccessType, OidDefinition, OidValueType};
use crate::snmp::parsers::parse_val_as_str;
use crate::snmp::stcip::parsers::parse_stage_val_swarco;
use std::collections::HashMap;

pub struct SwarcoOidRegistry {
    oids: HashMap<&'static str, OidDefinition>,
}

impl SwarcoOidRegistry {
    pub fn new() -> Self {
        let mut oids: HashMap<&'static str, OidDefinition> = HashMap::new();

        oids.insert(
            "1.3.6.1.4.1.1618.3.7.2.11.2",
            OidDefinition {
                name: "SwarcoUTCTrafftechPhaseStatus",
                parser: parse_stage_val_swarco,
                value_type: OidValueType::Unsigned32,
                access: AccessType::ReadWrite,
            },
        );

        oids.insert(
            "1.3.6.1.4.1.1618.3.7.2.1.2",
            OidDefinition {
                name: "swarcoUTCTrafftechPlanCurrent",
                parser: parse_val_as_str,
                value_type: OidValueType::Unsigned32,
                access: AccessType::ReadOnly,
            },
        );

        oids.insert(
            "1.3.6.1.4.1.1618.3.6.2.1.2",
            OidDefinition {
                name: "swarcoUTCStatusEquipment",
                parser: parse_val_as_str,
                value_type: OidValueType::Unsigned32,
                access: AccessType::ReadOnly,
            },
        );

        Self { oids: oids }
    }

    pub fn get(&self, oid: &str) -> Option<&OidDefinition> {
        self.oids.get(oid)
    }
}


// /// Текущая фаза
// pub const SwarcoUTCTrafftechPhaseStatus: OidDef = OidDef {
//     name: "SwarcoUTCTrafftechPhaseStatus",
//     // oid_as_str: &[1, 3, 6, 1, 4, 1, 1618, 3, 7, 2, 11, 2],
//     oid: oid!(1, 3, 6, 1, 4, 1, 1618, 3, 7, 2, 11, 2),
//     oid_as_str: "1.3.6.1.4.1.1618.3.7.2.11.2",
//     description: "Текущая фаза",
//     access: AccessType::ReadOnly,
// };
