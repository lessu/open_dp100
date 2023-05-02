use std::convert::TryFrom;

use crc16::*;

use crate::{opcode::OpCode,  data::{SystemInfo, BasicInfo, DeviceInfo, BasicSet, ScanOut, SerialOut, OperationResult, OpResult, OutputState}, error::OpenDP100Error};
use endianness::{read_u16,read_i16, ByteOrder};
use std::convert::TryInto;

pub trait Operational<const SIZE:usize> : Sized {
    fn to_data(&self)->[u8;SIZE];
    fn from_data(data:&[u8]) -> Result<Self,FrameError>;
}

#[derive(Debug)]
pub enum FrameError {
    UnknownHeader,
    InvalidCrc,
    DataTooLong,
    InvalidOpCode,
    InvalidPayload
}

pub struct Frame{
    pub op_code: OpCode,
    pub serial_num:u8,
    pub op_data: [u8;64],
    pub op_data_len:usize
}

impl Frame{
    pub fn empty() -> Self{
        Frame{
            op_code: OpCode::BasicInfo,
            serial_num:0,
            op_data: [0;64],
            op_data_len:0
        }
    }
    pub fn new(op_code: OpCode,data:&[u8]) -> Self{
        let mut r = Frame{
            op_code: op_code,
            serial_num:0,
            op_data:[0;64],
            op_data_len:data.len()
        };
        assert!(data.len() <  r.op_data.len());

        r.op_data[0..data.len()].copy_from_slice(data);

        r
    }
    pub fn data<'a>(&'a self)->&'a [u8]{
        &self.op_data[0..self.op_data_len]
    }

    pub fn append_data(&mut self,data:&[u8]) -> Result<(),OpenDP100Error>{
        if data.len() + self.op_data_len > self.op_data.len(){
            return Err(OpenDP100Error::INVALID_PARAM);
        }
        self.op_data[self.op_data_len..self.op_data_len+data.len()].copy_from_slice(data);
        self.op_data_len = self.op_data_len + data.len();
        Ok(())
    }
}

pub fn deserialize_in_frame(buffer:&[u8;64],frame:&mut Frame)->Result<(), FrameError>{
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


pub fn serialize_out_frame<'a>(frame:&Frame,buffer:&mut [u8;64]){
    buffer[0] = 0xfb;
    buffer[1] = frame.op_code.clone() as u8;
    buffer[2] = 0x00; // serial_num

    let data = frame.data();
    let data_len = data.len();
    // know length
    buffer[3] = data_len as u8;

    // data copy
    buffer[4..(4+data_len)].copy_from_slice(data);

    //crc
    let crc = State::<MODBUS>::calculate(&buffer[0..(4 + data_len as usize)]);
    buffer[4+data_len as usize] = crc as u8;
    buffer[4+data_len as usize + 1] = (crc >> 8) as u8;
}

// 0x10 DeviceInfo
impl Operational<40> for DeviceInfo {
    fn from_data(data: &[u8]) -> Result<Self,FrameError> {
        if data.len() != std::mem::size_of::<DeviceInfo>() { 
            return Err(FrameError::InvalidPayload);
        }
        Ok(DeviceInfo {
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

    fn to_data(&self) -> [u8;40]{
        return [0;40];
    }

}

// 0x30 OpCode::BasicInfo
impl Operational<16> for BasicInfo {

    fn from_data(data: &[u8]) -> Result<Self,FrameError> {
        const SIZE:usize = 16;

        if data.len() != SIZE { 
            return Err(FrameError::InvalidPayload)
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
    fn to_data(&self) -> [u8;16]{
        return [0;16];
    }
}

// 0x35 OpCode::BasicSet
impl Operational<10> for BasicSet {

    fn to_data(&self) -> [u8;10] {
        let mut data = [0u8;10];

        data[0] = self.index;
        data[1] = self.state.clone() as u8;
        data[2..4].copy_from_slice(&self.vo_set.to_le_bytes());
        data[4..6].copy_from_slice(&self.io_set.to_le_bytes());
        data[6..8].copy_from_slice(&self.ovp_set.to_le_bytes());
        data[8..10].copy_from_slice(&self.ocp_set.to_le_bytes());

        data
    }
    fn from_data(data: &[u8]) -> Result<Self,FrameError> {
        // 根据你的需求，从data中解析出相应的值
        Ok(BasicSet {
            // 初始化字段
            index:data[0],
            state: OutputState::from( data[1] ),
            vo_set: read_u16(&data[2..4], ByteOrder::LittleEndian).unwrap(),
            io_set: read_u16(&data[4..6], ByteOrder::LittleEndian).unwrap(),
            ovp_set: read_u16(&data[4..8], ByteOrder::LittleEndian).unwrap(),
            ocp_set: read_u16(&data[8..10], ByteOrder::LittleEndian).unwrap()
        })
    }
}

// 0x40 OpCode::SystemInfo
impl Operational<6> for SystemInfo {
    fn to_data(&self) -> [u8;6]{
        return [0;6];
    }
    fn from_data(data: &[u8]) -> Result<Self,FrameError> {
        const SIZE:usize = 6;

        if data.len() != SIZE {
            return Err(FrameError::InvalidPayload);
        }
        Ok(SystemInfo {
            blk_lev: data[0] as i8,
            opp: read_u16(&data[1..3], ByteOrder::LittleEndian).unwrap(),
            opt: read_u16(&data[3..5], ByteOrder::LittleEndian).unwrap(),
            vol_kev: data[5] as i8,
        })
    }
}
// 0x50 OpCode::ScanOut
impl Operational<12> for ScanOut {
    fn from_data(_data: &[u8]) -> Result<Self,FrameError> {
        Err(FrameError::InvalidPayload)
    }
    fn to_data(&self) -> [u8;12] {
        let mut data = [0u8;12];
        data[0] = self.on_off;
        data[1..3].copy_from_slice(&self.on_time.to_le_bytes());
        data[3..5].copy_from_slice(&self.out_val.to_le_bytes());
        data[5] = self.scan_mode;
        data[6..8].copy_from_slice(&self.start.to_le_bytes());
        data[8..10].copy_from_slice(&self.end.to_le_bytes());
        data[10..12].copy_from_slice(&self.step.to_le_bytes());
        data
    }
}


// 0x55 OpCode::SerialOut
impl Operational<10> for SerialOut {
    fn from_data(_data: &[u8]) -> Result<Self,FrameError> {
        Err(FrameError::InvalidPayload)
    }
    fn to_data(&self) -> [u8;10] {
        let mut data = [0u8;10];

        data[0] = self.on_off;
        data[1..3].copy_from_slice(&self.on_time.to_le_bytes());
        data[3] = self.ser_start;
        data[4] = self.ser_end;
        data[5..7].copy_from_slice(&self.ser_vi.to_le_bytes());
        data[7..9].copy_from_slice(&self.ser_vo.to_le_bytes());
        data[9] = self.cycle_times;

        data
    }
}

impl Operational<1> for OperationResult {
    fn to_data(&self)->[u8;1] {
        return [self.result.clone() as u8];
    }

    fn from_data(data:&[u8]) -> Result<Self,FrameError> {
        Ok(OperationResult {
            result: OpResult::from(data[0])
        })
    }
    
}