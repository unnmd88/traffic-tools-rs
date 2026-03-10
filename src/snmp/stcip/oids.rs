use crate::snmp::models::{ OidDef, AccessType };

/// Текущая фаза
pub const SwarcoUTCTrafftechPhaseStatus: OidDef = OidDef {
    name: "currentPhase",
    oid: &[1, 3, 6, 1, 4, 1, 1618, 3, 7, 2, 11, 2],
    description: "Текущая фаза",
    access: AccessType::ReadOnly,
};