mod protocol;
use protocol::ble;

mod ahakey;

use std::error::Error;

use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::platform::{ Manager };
use btleplug::api::{ Central, Manager as _, RetrievePeripheralsOptions };

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
            ])
        }
    ).await?;

    let peripheral = ble::find_device(devices).await?.unwrap();
    ble::find_characteristic(&peripheral).await?;
    let listener = ble::subscribe_to_notifications(&peripheral, 0x7344).await?;
    // read_from_char(&peripheral).await?;
    let data = [0xAA, 0xBB, 0x00, 0xCC, 0xDD];
    ble::write_to_char(&peripheral, 0x7343, &data).await?;

    tokio::signal::ctrl_c().await?;
    listener.abort();
    Ok(())
}