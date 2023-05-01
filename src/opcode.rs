use std::convert::TryFrom;

#[derive(Debug)]
pub enum OpCode {
    DeviceInfo = 0x10,
    FirmInfo = 0x11,
    StartTrans = 0x12,
    DataTrans = 0x13,
    EndTrans = 0x14,
    DevUpgrade = 0x15,
    BasicInfo = 0x30,
    BasicSet = 0x35,
    SystemInfo = 0x40,
    SystemSet = 0x45,
    ScanOut = 0x50,
    SerialOut = 0x55,
    Disconnect = 0x80,
}

impl TryFrom<u8> for OpCode {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x10 => Ok(OpCode::DeviceInfo),
            0x11 => Ok(OpCode::FirmInfo),
            0x12 => Ok(OpCode::StartTrans),
            0x13 => Ok(OpCode::DataTrans),
            0x14 => Ok(OpCode::EndTrans),
            0x15 => Ok(OpCode::DevUpgrade),
            0x30 => Ok(OpCode::BasicInfo),
            0x35 => Ok(OpCode::BasicSet),
            0x40 => Ok(OpCode::SystemInfo),
            0x45 => Ok(OpCode::SystemSet),
            0x50 => Ok(OpCode::ScanOut),
            0x55 => Ok(OpCode::SerialOut),
            0x80 => Ok(OpCode::Disconnect),
            _ => Err(format!("Invalid op code: {}", value)),
        }
    }
}