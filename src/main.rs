use std::error::Error;

use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::platform::{Manager};
use btleplug::api::{Central, Manager as _, Peripheral as _, RetrievePeripheralsOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;
    
    let adapter  = manager.adapters().await?;
    let central = adapter.into_iter().nth(0).expect("No Bluetooth adapters found");

    let devices = central.retrieve_peripherals(
        RetrievePeripheralsOptions {
            identifiers: None, 
            services: Some(vec![
                uuid_from_u16(0x7340),
                uuid_from_u16(0x1812),
            ])
        }
    ).await?;

    for peripheral in devices {
        let Some(props) = peripheral.properties().await? else {
            continue;
        }; 

        let name = props.local_name.unwrap_or_default();
        let lower = name.to_lowercase();
        if !lower.starts_with("ahakey") && !lower.starts_with("vibe code") {
            continue;
        }

        println!("Connected to the device.");

        println!("{:#?}", peripheral.properties().await.unwrap().unwrap());
    }

    Ok(())
}