use std::error::Error;

use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::platform::{Manager, Peripheral};
use btleplug::api::{Central, Manager as _, Peripheral as _, RetrievePeripheralsOptions, CharPropFlags};

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

async fn find_characteristic(peripheral: &Peripheral) -> Result<(), Box<dyn Error>> {
    peripheral.connect().await?;
    peripheral.discover_services().await?;
    for service in peripheral.services() {
        println!(
            "Service UUID: {}, Primary: {}",
            service.uuid,
            service.primary
        );

        for characteristic in service.characteristics {
            println!("  Characteristic: UUID {}, Properties: {:?}", characteristic.uuid, characteristic.properties);
        }
    }

    Ok(())
    
}

async fn read_from_char(peripheral: &Peripheral) -> Result<(), Box<dyn Error>> {
    if let Some(characteristic) = peripheral.characteristics().into_iter().find(|c| c.properties.contains(CharPropFlags::READ)) {
        let value  = peripheral.read(&characteristic).await?;
        println!("Read value: {:?}", value);
    } else {
        return Err(btleplug::Error::PermissionDenied.into());
    };

    Ok(())
}

async fn write_to_char(peripheral: &Peripheral) -> Result<(), Box<dyn Error>> {
    if let Some(characteristic) = peripheral.characteristics().into_iter().find(|c| c.properties.contains(CharPropFlags::WRITE)) {
        let data = [0xAA, 0xBB, 0x00, 0xCC, 0xDD];
        peripheral.write(&characteristic, &data, btleplug::api::WriteType::WithResponse).await?;
        println!("Wrote data successfully.");
    };

    Ok(())
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
            ])
        }
    ).await?;

    let peripheral = find_device(devices).await?.unwrap();
    find_characteristic(&peripheral).await?;
    // read_from_char(&peripheral).await?;
    write_to_char(&peripheral).await?;
    Ok(())
}