use std::convert::TryFrom;

#[derive(Debug,Clone)]
pub struct DeviceInfo {
    pub dev_type: [u8; 16],
    pub hdw_ver: u16,
    pub app_ver: u16,
    pub boot_ver: u16,
    pub run_area: u16,
    pub dev_sn: [u8; 12],
    pub year: u16,
    pub moon: u8,
    pub day: u8,
}

#[derive(Debug,Clone,PartialEq)]
#[repr(u8)]
pub enum OutputState{
    On = 0x01,
    Off = 0x00
}
impl From<u8> for OutputState {
    fn from(value: u8) -> Self {
        match value {
            0x00 => OutputState::Off,
            0x01 => OutputState::On,
            _ => OutputState::Off,
        }
    }
}

#[derive(Debug,Clone)]
pub struct BasicSet {
    pub index: u8,
    pub state: OutputState,
    pub vo_set: u16,
    pub io_set: u16,
    pub ovp_set: u16,
    pub ocp_set: u16
}

#[derive(Debug,Clone)]
pub struct BasicInfo {
    // unit mV
    pub vin: u16,
    // unit mV
    pub vout: u16,
    // ?
    pub iout: u16,
    // unit mV
    pub vo_max: u16,
    // 100 m degree C
    pub temp1: u16,
    // 100 m degree C
    pub temp2: i16,
    // unit mV
    pub dc_5v: u16,
    pub out_mode: u8,
    pub work_st: u8,
}


#[derive(Debug,Clone)]
pub struct SystemInfo {
    pub blk_lev: i8,
    pub opp: u16,
    pub opt: u16,
    pub vol_kev: i8,
}


// 0x50 SCAN_OUT
#[derive(Debug,Clone)]
pub struct ScanOut {
    pub on_off: u8,
    pub on_time: u16,
    pub out_val: u16,
    pub scan_mode: u8,
    pub start: u16,
    pub end: u16,
    pub step: u16,
}


// 0x55 SERIAL_OUT

#[derive(Debug,Clone)]
pub struct SerialOut {
    pub on_off: u8,
    pub on_time: u16,
    pub ser_start: u8,
    pub ser_end: u8,
    pub ser_vi: u16,
    pub ser_vo: u16,
    pub cycle_times: u8,
}

#[derive(Debug,Clone)]
#[repr(u8)]
pub enum OpResult{
    Failed  = 0x00,
    Success = 0x01,
    Unknown
}

impl From<u8> for OpResult {
    fn from(value: u8) -> Self {
        match value {
            0x00 => OpResult::Failed,
            0x01 => OpResult::Success,
            _ => OpResult::Unknown,
        }
    }
}

#[derive(Debug,Clone)]
pub struct OperationResult {
    pub result: OpResult,
}