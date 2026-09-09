use std::error::Error;

use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::platform::{Manager, Peripheral};
use btleplug::api::{Central, Manager as _, Peripheral as _, RetrievePeripheralsOptions};

async fn find_device(device: Vec<Peripheral>) -> Result<Option<Peripheral>, Box<dyn Error>> {
    Ok(None)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;
    
    let adapter  = manager.adapters().await?;
    let central = adapter.into_iter().nth(0).expect("No Bluetooth adapters found");

    let devices = central.retrieve_peripherals(
        RetrievePeripheralsOptions {
            identifiers: None, 
            services: None
        }
    ).await?;

    for peripheral in devices {
        let Some(props) = peripheral.properties().await? else {
            continue;
        }; 

        let name = props.local_name.as_deref().unwrap_or_default();
        let lower = name.to_lowercase();
        if !lower.starts_with("ahakey") && !lower.starts_with("vibe code") {
            continue;
        }
    }
    

    Ok(())
}