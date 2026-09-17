#[derive(Clone, Debug)]
pub(crate) struct HardwareState {
    pub(crate) work_mode: u8,
    pub(crate) light_mode: u8,
    pub(crate) battery: u8,
    pub(crate) specific: SpecificState,
}

#[derive(Clone, Debug)]
pub(crate) enum SpecificState {
    X1(X1ExtraState),
}

#[derive(Clone, Debug)]
pub(crate) struct X1ExtraState {
    pub(crate) sw_state: SwitchState,
    pub(crate) brightness: u8
}

#[derive(Clone, Debug)]
pub(crate) enum SwitchState {
    SWITCHUP,
    SWITCHDOWN,
    UNKNOWN
}

impl SwitchState {
    fn from_byte(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::SWITCHUP),
            1 => Some(Self::SWITCHDOWN),
            2 => Some(Self::UNKNOWN),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct X1State {
    work_mode: u8,
    light_mode: u8,
    sw_state: SwitchState,
    battery: u8,
    brightness: u8
}

impl X1State {
    pub(crate) fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() != 13
            || !data.starts_with(&[0xAA, 0xBB, 0x00])
            || !data.ends_with(&[0xCC, 0xDD])
        {
            return None;
        }
        
        let sw_state = SwitchState::from_byte(data[9])?;

        Some (Self {
            battery: data[3],
            work_mode: data[7],
            light_mode: data[8],
            sw_state,
            brightness: data[10],
        })
    }
}

impl From<X1State> for HardwareState {
    fn from(value: X1State) -> Self {
        Self { 
            work_mode: value.work_mode,
            light_mode: value.light_mode,
            battery: value.battery, 
            specific: SpecificState::X1(X1ExtraState { 
                sw_state: value.sw_state, 
                brightness: value.brightness,
            })
        }
    }
}
