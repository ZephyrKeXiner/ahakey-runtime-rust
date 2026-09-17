use super::hardware::Hardware;

pub(crate) struct AhaKey {
    hardware: Box<dyn Hardware>,
}
