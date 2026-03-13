use async_snmp::{ 
    Auth, 
    Client, 
    Value, 
    oid, 
    Oid
 };
use std::io::{self, Write};
use std::time::Duration;
use traffic_tools_rs::snmp::models::OidResult;
use traffic_tools_rs::snmp::stcip::parsers::parse_stage_val_swarco;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Проверка snmp запросов (интерактивный)");

    let stcip = Client::builder("localhost:1161", Auth::v2c("public"))
        .timeout(Duration::from_secs(3))
        .connect()
        .await?;

    let phase_oid = oid!(1, 3, 6, 1, 4, 1, 1618, 3, 7, 2, 11, 2, 0);

    loop {
        print!("\n> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            println!("👋 Выход...");
            break;
        }

        // Обрабатываем ошибку внутри цикла
        match stcip.get(&phase_oid).await {
            Ok(result) => {
                // let response = SnmpResponse::new(
                //     result.value,
                //     "PhaseStatus".to_string(),
                //     phase_oid.to_string(),
                // );
                let value = result.value;
                
                // Вычисляем всё до создания структуры
                let raw_as_str = match value.as_str() {
                    Some(s) => s.to_string(),
                    None => format!("{:?}", value),  // или hex::encode
                };
                let parsed = parse_stage_val_swarco(&value);
                let response = OidResult::new(
                    "swarcoUTCTrafftechPhaseStatus",
                    "1.3.6.1.4.1.1618.3.7.2.11.2",
                    value,
                    parse_stage_val_swarco,
                );
                // println!("swarcoUTCTrafftechPhaseStatus: {:?}", response.value);
                println!("{}", serde_json::to_string_pretty(&response)?);
            }
            Err(e) => {
                println!("Ошибка: {}", e);
            }
        }
        
        println!("  Response   : {}", input);
        println!("----------------------------------------");
    }
    
    Ok(())
}

