use std::error::Error;

use btleplug::api::bleuuid::uuid_from_u16;
use tokio::time::{sleep, Duration};
use btleplug::platform::{Adapter, Manager, Peripheral};
use btleplug::api::{self, Central, Manager as _, Peripheral as _, RetrievePeripheralsOptions, ScanFilter, CharPropFlags, WriteType};
use futures_util::StreamExt;

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
    }



    Ok(())
}