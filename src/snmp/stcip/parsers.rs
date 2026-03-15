use async_snmp::Value;

pub fn parse_stage_val_swarco(stage: Value) -> Option<String> {

    match stage.as_u32() {
        Some(stage_val) => {
            if stage_val == 1 {
                Some("8".to_string())
            }
            else if stage_val >= 2 && stage_val <= 8 {
                    Some((stage_val - 1).to_string())
                }
            else {
                None
            }
        }
        None => None
    }
}
