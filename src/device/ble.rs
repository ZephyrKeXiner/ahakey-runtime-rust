use std::error::Error;

use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::platform::{ Peripheral };
use btleplug::api::{ Peripheral as _, CharPropFlags };
use futures_util::StreamExt;

pub(crate) async fn find_device(devices: Vec<Peripheral>) -> Result<Option<Peripheral>, Box<dyn Error>> {
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

pub(crate) async fn find_characteristic(peripheral: &Peripheral) -> Result<(), Box<dyn Error>> {
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

pub(crate) async fn read_from_char(peripheral: &Peripheral) -> Result<(), Box<dyn Error>> {
    if let Some(characteristic) = peripheral.characteristics().into_iter().find(|c| c.properties.contains(CharPropFlags::READ)) {
        let value  = peripheral.read(&characteristic).await?;
        println!("Read value: {:?}", value);
    } else {
        return Err(btleplug::Error::PermissionDenied.into());
    };

    Ok(())
}

pub(crate) async fn write_to_char(peripheral: &Peripheral, data: &[u8]) -> Result<(), Box<dyn Error>> {
    if let Some(characteristic) = peripheral.characteristics().into_iter().find(|c| c.uuid == uuid_from_u16(0x7343)) {
        peripheral.write(&characteristic, &data, btleplug::api::WriteType::WithResponse).await?;
        println!("Wrote data successfully.");
    };

    Ok(())
}

pub(crate) async fn subscribe_to_notifications(peripheral: &Peripheral) -> Result<tokio::task::JoinHandle<()>, Box<dyn Error>> {
    if let Some(characteristic) = peripheral.characteristics().into_iter().find(|c| c.uuid == uuid_from_u16(0x7344)) {
        println!("Subscribing to characteristic {}", characteristic.uuid);
        peripheral.subscribe(&characteristic).await?;

        let mut notification_stream = peripheral.notifications().await?;
        
        let listener = tokio::spawn(async move {
            while let Some(data) = notification_stream.next().await {
                println!(
                    "Received notification from UUID {}: {:?}",
                    data.uuid,
                    data.value
                );
            }       
        }); 
        Ok(listener)
    } else {
        Err(btleplug::Error::NotSupported("Failed to subscribe the characteristic".to_string()).into())
    }
    
}