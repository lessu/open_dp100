use crc16::*;
use crate::opcode::OpCode;

pub trait OperationOut{
    fn op_code(&self)->OpCode;
    fn copy_data(&self,data:&mut [u8]) -> u8;
}

pub fn serialize_out_frame<'a>(op:&dyn OperationOut,buffer:&mut [u8;64]){
    buffer[0] = 0xfb;
    buffer[1] = op.op_code() as u8;
    buffer[2] = 0x00; // serial_num

    // data copy
    let data_len = op.copy_data(&mut buffer[4..]);
    // know length
    buffer[3] = data_len;

    //crc
    let crc = State::<MODBUS>::calculate(&buffer[0..(4 + data_len as usize)]);
    buffer[4+data_len as usize] = crc as u8;
    buffer[4+data_len as usize + 1] = (crc >> 8) as u8;

}

pub struct DeviceInfoOut;
impl OperationOut for DeviceInfoOut{
    fn op_code(&self)->OpCode{
        // return 0x10;
        return OpCode::DeviceInfo;
    }
    fn copy_data(&self,_data:&mut [u8]) -> u8{
        return 0;
    }
}

pub struct FirmInfoOut;
impl OperationOut for FirmInfoOut{
    fn op_code(&self)->OpCode{
        // return 0x11;
        return OpCode::FirmInfo;
    }
    fn copy_data(&self,_data:&mut [u8]) -> u8{
        return 0;
    }
}

pub struct StartTransOut;
impl OperationOut for StartTransOut{
    fn op_code(&self)->OpCode{
        // return 0x12;
        return OpCode::StartTrans;
    }
    fn copy_data(&self,_data:&mut [u8]) -> u8{
        return 0;
    }
}

pub struct DataTransOut;
impl OperationOut for DataTransOut{
    fn op_code(&self)->OpCode{
        // return 0x13;
        return OpCode::DataTrans;
    }
    fn copy_data(&self,_data:&mut [u8]) -> u8{
        return 0;
    }
}

pub struct EndTransOut;
impl OperationOut for EndTransOut{
    fn op_code(&self)->OpCode{
        // return 0x14;
        return OpCode::EndTrans;
    }
    fn copy_data(&self,_data:&mut [u8]) -> u8{
        return 0;
    }
}

pub struct DevUpgradeOut;
impl OperationOut for DevUpgradeOut{
    fn op_code(&self)->OpCode{
        // return 0x15;
        return OpCode::DevUpgrade;
    }
    fn copy_data(&self,_data:&mut [u8]) -> u8{
        return 0;
    }
}

pub struct BasicInfoOut;
impl OperationOut for BasicInfoOut{
    fn op_code(&self)->OpCode{
        return OpCode::BasicInfo;
    }
    fn copy_data(&self,_data:&mut [u8]) -> u8{
        return 0;
    }
}

pub struct BasicSetOut {
    pub index: u8,
    pub state: u8,
    pub vo_set: u16,
    pub io_set: u16,
    pub ovp_set: u16,
    pub ocp_set: i16,
}

impl OperationOut for BasicSetOut {
    fn op_code(&self) -> OpCode {
        return OpCode::BasicSet;
    }
    fn copy_data(&self, data: &mut [u8]) -> u8 {
        if data.len() < std::mem::size_of::<BasicSetOut>() {
            return 0;
        }        
        data[0] = self.index;
        data[1] = self.state;
        data[2..4].copy_from_slice(&self.vo_set.to_le_bytes());
        data[4..6].copy_from_slice(&self.io_set.to_le_bytes());
        data[6..8].copy_from_slice(&self.ovp_set.to_le_bytes());
        data[8..10].copy_from_slice(&self.ocp_set.to_le_bytes());
        std::mem::size_of::<BasicSetOut>() as u8
    }
}

pub struct SystemInfoOut;
impl OperationOut for SystemInfoOut{
    fn op_code(&self)->OpCode{
        return OpCode::SystemInfo;
    }
    fn copy_data(&self,_data:&mut [u8]) -> u8{
        return 0;
    }
}

pub struct SystemSetOut;
impl OperationOut for SystemSetOut{    
    fn op_code(&self)->OpCode{
        return OpCode::SystemSet;
    }
    fn copy_data(&self,_data:&mut [u8]) -> u8{
        return 0;
    }
}
// 0x50 SCAN_OUT
pub struct ScanOutOut {
    pub on_off: u8,
    pub on_time: u16,
    pub out_val: u16,
    pub scan_mode: u8,
    pub start: u16,
    pub end: u16,
    pub step: u16,
}

impl OperationOut for ScanOutOut {
    fn op_code(&self) -> OpCode {
        return OpCode::ScanOut;
    }
    fn copy_data(&self, data: &mut [u8]) -> u8 {
        if data.len() < std::mem::size_of::<ScanOutOut>() {
            return 0;
        }     
        data[0] = self.on_off;
        data[1..3].copy_from_slice(&self.on_time.to_le_bytes());
        data[3..5].copy_from_slice(&self.out_val.to_le_bytes());
        data[5] = self.scan_mode;
        data[6..8].copy_from_slice(&self.start.to_le_bytes());
        data[8..10].copy_from_slice(&self.end.to_le_bytes());
        data[10..12].copy_from_slice(&self.step.to_le_bytes());
        std::mem::size_of::<ScanOutOut>() as u8 as u8
    }
}

// 0x55 SERIAL_OUT
pub struct SerialOutOut {
    pub on_off: u8,
    pub on_time: u16,
    pub ser_start: u8,
    pub ser_end: u8,
    pub ser_vi: u16,
    pub ser_vo: u16,
    pub cycle_times: u8,
}

impl OperationOut for SerialOutOut {
    fn op_code(&self) -> OpCode {
        return OpCode::SerialOut;
    }
    fn copy_data(&self, data: &mut [u8]) -> u8 {
        if data.len() < std::mem::size_of::<SerialOutOut>() {
            return 0;
        }
        data[0] = self.on_off;
        data[1..3].copy_from_slice(&self.on_time.to_le_bytes());
        data[3] = self.ser_start;
        data[4] = self.ser_end;
        data[5..7].copy_from_slice(&self.ser_vi.to_le_bytes());
        data[7..9].copy_from_slice(&self.ser_vo.to_le_bytes());
        data[9] = self.cycle_times;
        std::mem::size_of::<SerialOutOut>() as u8 as u8
    }
}

pub struct DisconnectOut;
impl OperationOut for DisconnectOut{
    fn op_code(&self)->OpCode{
        return OpCode::Disconnect;
    }
    fn copy_data(&self,_data:&mut [u8]) -> u8{
        //TODO
        0
    }
}