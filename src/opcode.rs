use std::convert::TryFrom;

#[derive(Debug)]
#[derive(Clone)]
#[repr(u8)]
pub enum OpCode {
    None = 0x00,
    DeviceInfo = 0x10,
    BasicInfo = 0x30,
    BasicSet = 0x35,
    SystemInfo = 0x40,
    ScanOut = 0x50,
    SerialOut = 0x55,
}

impl TryFrom<u8> for OpCode {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(OpCode::None),
            0x10 => Ok(OpCode::DeviceInfo),
            0x30 => Ok(OpCode::BasicInfo),
            0x35 => Ok(OpCode::BasicSet),
            0x40 => Ok(OpCode::SystemInfo),
            0x50 => Ok(OpCode::ScanOut),
            0x55 => Ok(OpCode::SerialOut),
            _ => Err(format!("Invalid op code: {}", value)),
        }
    }
}