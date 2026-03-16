use chrono::Utc;
use async_snmp::{ 
    Auth, 
    Client, 
    Oid
 };
use std::io::{self, Write};
use std::iter::zip;
use std::str::FromStr;
use std::time::Duration;
use traffic_tools_rs::snmp::models::{OidResult, OidDefinition, SnmpResponse};
use traffic_tools_rs::snmp::stcip::oids::SwarcoOidRegistry;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Проверка snmp запросов (интерактивный)");

    // Список оидов от пользователя
    let oids_from_user = vec! (
        "1.3.6.1.4.1.1618.3.7.2.11.2".to_string(),
        "1.3.6.1.4.1.1618.3.7.2.1.2".to_string(),
        "1.3.6.1.4.1.1618.3.6.2.1.2".to_string(),
    );

    let host_id = 1u8;
    let host = "localhost:1161";

    // На основе пользовательских оидов формируем вектор настоящих Oid для запроса.
    let oids: Vec<Oid> = oids_from_user
        .iter()
        .map(|oid| Oid::from_str(oid))
        .collect::<Result<Vec<Oid>, _>>()?;

    // Создаем хешмап с реестром всех оидов сварки
    let oid_lib = SwarcoOidRegistry::new();

    // Создаем вектор с OidDef, чтобы после response распарсить каждый полученный оид и создать OidResult.
    let oids_def: Vec<&OidDefinition> = oids_from_user
        .iter()
        .filter_map(|oid_str| oid_lib.get(oid_str))
        .collect();

    let client = Client::builder("localhost:1161", Auth::v2c("public"))
        .timeout(Duration::from_secs(3))
        .connect()
        .await?;

 

    // let phase_oid = oid!(1, 3, 6, 1, 4, 1, 1618, 3, 7, 2, 11, 2, 0);

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
        // let mut response_to_client: HashMap<String, OidResult> = HashMap::new();
        
        let varbinds = client.get_many(&oids).await?;

        let mut vardinds_res: Vec<OidResult> = Vec::new();


        for (varbind, oid_def) in zip(&varbinds, &oids_def) {
            vardinds_res.push(OidResult::new(&oid_def.name, varbind.oid.to_string(), varbind.value.clone(), oid_def.parser));
            // response_to_client.insert(
            //     varbind.oid.to_string(), 
            //     OidResult::new(&oid_def.name, varbind.oid.to_string(), varbind.value.clone(), oid_def.parser)
            // );
        }
        let response_to_client2 = SnmpResponse {
            host: host.to_string(),
            host_id: host_id,
            oids_request: oids_from_user.clone(),
            oids_response: vardinds_res,
            timestamp: Utc::now().to_rfc3339(),
        };

        // let response_to_client2 = json!({
        //     "host_id": host_id,
        //     "host_address": host,
        //     "oids": json! ({
        //         "request": oids_from_user,
        //         "response": vardinds_res
        //     }),
        // });

        println!("{}", serde_json::to_string_pretty(&response_to_client2)?);
        // println!("  Response   : {}", serde_json::response_to_client);
        println!("----------------------------------------");
    }
    
    Ok(())
}

