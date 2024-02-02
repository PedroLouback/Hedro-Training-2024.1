use crate::services::messages::RMQMessage;

pub trait BridgeService {
    fn exec(&self, msg: &RMQMessage) -> Result<(), ()>;
}

pub struct BridgeServiceImpl {}

impl BridgeServiceImpl {
    pub fn new() -> Self {
        BridgeServiceImpl {}
    }
}

impl BridgeService for BridgeServiceImpl {
    fn exec(&self, _msg: &RMQMessage) -> Result<(), ()> {
        Ok(())
    }
}
