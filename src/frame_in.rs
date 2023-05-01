use std::convert::TryFrom;

use crc16::*;

use crate::opcode::OpCode;
use endianness::{read_u16,read_i16, ByteOrder};
use std::convert::TryInto;

pub trait OperationIn:Sized {
    fn from_data(data:&[u8]) -> Result<Self,()>;
}

#[derive(Debug)]
pub enum FrameError {
    UnknownHeader,
    InvalidCrc,
    DataTooLong,
    InvalidOpCode
}

pub struct InFrame{
    pub op_code: OpCode,
    pub serial_num:u8,
    pub op_data: [u8;64],
    pub op_data_len:usize
}
impl InFrame{
    pub fn empty() -> Self{
        InFrame{
            op_code: OpCode::FirmInfo,
            serial_num:0,
            op_data: [0;64],
            op_data_len:0
        }
    }
    pub fn data<'a>(&'a self)->&'a [u8]{
        &self.op_data[0..self.op_data_len]
    }
}

pub fn deserialize_in_frame(buffer:&[u8;64],frame:&mut InFrame)->Result<(), FrameError>{
    if buffer[0] != 0xfa {
        // "Unknown"
        return Err(FrameError::UnknownHeader);
    }

    let res = OpCode::try_from(buffer[1]);
    if res.is_err() {
        return Err(FrameError::InvalidOpCode);
    }

    frame.op_code =  res.unwrap();
    frame.serial_num = buffer[2];
    let len = buffer[3] as usize;

    //crc
    let calc_crc = State::<MODBUS>::calculate(&buffer[0..len+4]);
    let recv_crc = read_u16(&buffer[4+len..4+len+2], ByteOrder::LittleEndian).unwrap();
    if calc_crc != recv_crc {
        return Err(FrameError::InvalidCrc);
    }
    
    // data copy
    if len > frame.op_data.len() {
        return Err(FrameError::DataTooLong);
    }
    frame.op_data[0..len].copy_from_slice(&buffer[4..4+len]);
    frame.op_data_len = len;
    Ok(())
}


pub struct DeviceInfoIn {
    dev_type: [u8; 16],
    hdw_ver: u16,
    app_ver: u16,
    boot_ver: u16,
    run_area: u16,
    dev_sn: [u8; 12],
    year: u16,
    moon: u8,
    day: u8,
}

impl OperationIn for DeviceInfoIn {
    fn from_data(data: &[u8]) -> Result<Self,()> {
        if data.len() != std::mem::size_of::<DeviceInfoIn>() { 
            return Err(())
        }
        Ok(DeviceInfoIn {
            dev_type: data[0..16].try_into().unwrap(),
            hdw_ver: read_u16(&data[16..18], ByteOrder::LittleEndian).unwrap(),
            app_ver: read_u16(&data[18..20], ByteOrder::LittleEndian).unwrap(),
            boot_ver: read_u16(&data[20..22], ByteOrder::LittleEndian).unwrap(),
            run_area: read_u16(&data[22..24], ByteOrder::LittleEndian).unwrap(),
            dev_sn: data[24..36].try_into().unwrap(),
            year: read_u16(&data[36..38], ByteOrder::LittleEndian).unwrap(),
            moon: data[38],
            day: data[39]
        })
    }
}

pub struct FirmInfo {
    hdw_ver: u16,
    app_ver: u16,
    bin_crc16: u16,
    bin_size: u8,
    data_len: u8,
    dev_type: [u8; 16],
    encrypt_pos: u8,
    year: u16,
    moon: u8,
    day: u16,
}

impl OperationIn for FirmInfo {
    fn from_data(data: &[u8]) -> Result<Self,()> {
        const SIZE:usize = 30;

        if data.len() != SIZE { 
            return Err(())
        }
        Ok(FirmInfo {
            hdw_ver: read_u16(&data[0..2], ByteOrder::LittleEndian).unwrap(),
            app_ver: read_u16(&data[2..4], ByteOrder::LittleEndian).unwrap(),
            bin_crc16: read_u16(&data[4..6], ByteOrder::LittleEndian).unwrap(),
            bin_size: data[6],
            data_len: data[7],
            dev_type: data[8..24].try_into().unwrap(),
            encrypt_pos: data[24],
            year: read_u16(&data[25..27], ByteOrder::LittleEndian).unwrap(),
            moon: data[27],
            day: read_u16(&data[28..30], ByteOrder::LittleEndian).unwrap(),
        })
    }
}
pub struct StartTransIn {
    // 根据你的需求，添加相应的字段
}

impl OperationIn for StartTransIn {
    fn from_data(data: &[u8]) -> Result<Self,()> {
        // 根据你的需求，从data中解析出相应的值
        Ok(StartTransIn {
            // 初始化字段
        })
    }
}

pub struct DataTransIn {
    // 根据你的需求，添加相应的字段
}

impl OperationIn for DataTransIn {
    fn from_data(data: &[u8]) -> Result<Self,()> {
        // 根据你的需求，从data中解析出相应的值
        Ok(DataTransIn {
            // 初始化字段
        })
    }
}

pub struct EndTransIn {
    // 根据你的需求，添加相应的字段
}

impl OperationIn for EndTransIn {
    fn from_data(data: &[u8]) -> Result<Self,()> {
        // 根据你的需求，从data中解析出相应的值
        Ok(EndTransIn {
            // 初始化字段
        })
    }
}

pub struct DevUpgradeIn {
    // 根据你的需求，添加相应的字段
}

impl OperationIn for DevUpgradeIn {
    fn from_data(data: &[u8]) -> Result<Self,()> {
        // 根据你的需求，从data中解析出相应的值
        Ok(DevUpgradeIn {
            // 初始化字段
        })
    }
}
#[derive(Debug)]
pub struct BasicInfo {
    // unit mV
    vin: u16,
    // unit mV
    vout: u16,
    // ?
    iout: u16,
    // unit mV
    vo_max: u16,
    // 100 m degree C
    temp1: u16,
    // 100 m degree C
    temp2: i16,
    // unit mV
    dc_5v: u16,
    out_mode: u8,
    work_st: u8,
}

impl OperationIn for BasicInfo {

    fn from_data(data: &[u8]) -> Result<Self,()> {
        const SIZE:usize = 16;

        if data.len() != SIZE { 
            return Err(())
        }
        Ok(BasicInfo {
            vin: read_u16(&data[0..2], ByteOrder::LittleEndian).unwrap(),
            vout: read_u16(&data[2..4], ByteOrder::LittleEndian).unwrap(),
            iout: read_u16(&data[4..6], ByteOrder::LittleEndian).unwrap(),
            vo_max: read_u16(&data[6..8], ByteOrder::LittleEndian).unwrap(),
            temp1: read_u16(&data[8..10], ByteOrder::LittleEndian).unwrap(),
            temp2: read_i16(&data[10..12], ByteOrder::LittleEndian).unwrap(),
            dc_5v: read_u16(&data[12..14], ByteOrder::LittleEndian).unwrap(),
            out_mode: data[14],
            work_st: data[15],
        })
    }
}

pub struct BasicSetIn {
    // 根据你的需求，添加相应的字段
}

impl OperationIn for BasicSetIn {
    fn from_data(data: &[u8]) -> Result<Self,()> {
        // 根据你的需求，从data中解析出相应的值
        Ok(BasicSetIn {
            // 初始化字段
        })
    }
}

pub struct SystemInfo {
    blk_lev: i8,
    opp: u16,
    opt: u16,
    vol_kev: i8,
}

impl OperationIn for SystemInfo {
    fn from_data(data: &[u8]) -> Result<Self,()> {
        const SIZE:usize = 6;

        if data.len() != SIZE { 
            return Err(())
        }
        Ok(SystemInfo {
            blk_lev: data[0] as i8,
            opp: read_u16(&data[1..3], ByteOrder::LittleEndian).unwrap(),
            opt: read_u16(&data[3..5], ByteOrder::LittleEndian).unwrap(),
            vol_kev: data[5] as i8,
        })
    }
}

pub struct SystemSetIn {
    // 根据你的需求，添加相应的字段
}

impl OperationIn for SystemSetIn {
    fn from_data(data: &[u8]) -> Result<Self,()> {
        // 根据你的需求，从data中解析出相应的值
        Ok(SystemSetIn {
            // 初始化字段
        })
    }
}

pub struct ScanOutIn {
    // 根据你的需求，添加相应的字段
}

impl OperationIn for ScanOutIn {
    fn from_data(data: &[u8]) -> Result<Self,()> {
        // 根据你的需求，从data中解析出相应的值
        Ok(ScanOutIn {
            // 初始化字段
        })
    }
}

pub struct SerialOutIn {
    // 根据你的需求，添加相应的字段
}

impl OperationIn for SerialOutIn {
    fn from_data(data: &[u8]) -> Result<Self,()> {
        // 根据你的需求，从data中解析出相应的值
        Ok(SerialOutIn {
            // 初始化字段
        })
    }
}

pub struct DisconnectIn {
    // 根据你的需求，添加相应的字段
}

impl OperationIn for DisconnectIn {
    fn from_data(data: &[u8]) -> Result<Self,()> {
        // 根据你的需求，从data中解析出相应的值
        Ok(DisconnectIn {
            // 初始化字段
        })
    }
}