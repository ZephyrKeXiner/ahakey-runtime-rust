use crate::ahakey::state::{self, X1State};

use std::{os::macos::raw::stat, sync::Arc};

use btleplug::{api::Characteristic, platform::Peripheral};
use tokio::{sync::RwLock, task::JoinHandle};

pub(crate) struct ModelX1 {
    peripheral: Peripheral, 
    characteristic: Characteristic,
    state: Arc<RwLock<Option<X1State>>>, 
    listener: JoinHandle<()>,
}

impl ModelX1 {
    pub(crate) fn new(
        peripheral: Peripheral,
        characteristic: Characteristic,
    ) -> Self {
        
        Self {
            peripheral,
            characteristic,
            state: Arc::new(RwLock::new(None)),
            listener: None,
        }
    }
}
