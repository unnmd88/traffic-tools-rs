
enum Oids {
    swarcoUTCTrafftechPhaseStatus,

}
pub enum Access {
    ReadOnly,
    ReadWrite,
    WriteOnly,
}

struct OidData {
    identity: Oids,
    nums: Vec<u16>,
    access: Access,
    description: &str,
}