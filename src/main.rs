use std::error::Error;

use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::platform::{Manager, Peripheral};
use btleplug::api::{Central, Characteristic, Manager as _, Peripheral as _, RetrievePeripheralsOptions};

async fn find_device(devices: Vec<Peripheral>) -> Result<Option<Peripheral>, Box<dyn Error>> {

    for peripheral in devices {
        let Some(props) = peripheral.properties().await? else {
            continue;
        };

        let name = props.local_name.as_deref().unwrap_or_default();
        let lower = name.to_lowercase();
        if !lower.starts_with("ahakey") && !lower.starts_with("vibe code") {
            continue;
        }

        println!("{:#?}", props);
        return Ok(Some(peripheral));
    }

    Ok(None)
}

async fn find_characteristic(peripheral: Option<&Peripheral>) {
    let peripheral = peripheral.ok_or_else(|| btleplug::Error::DeviceNotFound)?;
}

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
                uuid_from_u16(0x7343),
                uuid_from_u16(0x7344),
            ])
        }
    ).await?;

    let peripheral = find_device(devices).await?;

    Ok(())
}