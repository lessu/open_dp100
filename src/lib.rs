use frame::{deserialize_in_frame,serialize_out_frame,Frame,Operational,FrameError};
use hidapi::{HidApi, HidDevice};

pub use error::OpenDP100Error;
pub use opcode::OpCode;

pub use data::{OutputState,BasicInfo,SystemInfo,DeviceInfo,BasicSet,OperationResult};

mod frame;
mod opcode;
mod error;
mod data;

const VID: u16 = 0x2e3c;
const PID: u16 = 0xaf01;
const READ_TIME_OUT_MS : i32 =200;

pub struct OpenDP100{
    hid_device:HidDevice,
}
macro_rules! session_impl {
    ($self:expr,$op_code:pat,$req:expr,$res:ident)=>{
        {
            let mut retry = 0;
            let response_res = loop{
                let frame = $self.session($req)?;

                match frame.op_code {
                    $op_code => {
                        break $res::from_data(frame.data());
                    }
                    _ => {
                        if retry == 3{
                            break Err(FrameError::InvalidOpCode);
                        }else{
                            retry = retry + 1;
                            continue;
                        }
                    }
                }
            };

            match response_res{
                Ok(response) => {
                    Ok(response)
                }
                Err(_)=>{
                    Err(OpenDP100Error::DEVICE)
                }
            }
        }
    }
}


impl OpenDP100{
    pub fn device_count() -> Result<usize,OpenDP100Error>{
        let api_res = HidApi::new();
        if api_res.is_err(){
            return Err(OpenDP100Error::DRIVER);
        }

        let mut count = 0;
        let api = api_res.unwrap();
        for device in api.device_list() {
            if device.vendor_id() == VID && device.product_id() == PID{
                count = count + 1;
            }
        }
        Ok(count)
    }

    pub fn new(device_idx:usize) -> Option<Self>{
        let api = HidApi::new().unwrap();
        let mut count = 0;
        let mut device_info = None;

        for device in api.device_list() {
            if device.vendor_id() == VID && device.product_id() == PID{
                if count == device_idx{
                    device_info = Some(device.clone());
                }
                count = count + 1;
            }
        }

        if device_info.is_none(){
            return None;
        }

        let device = device_info.unwrap().open_device(&api).unwrap();

        Some(Self{
            hid_device : device
        })
    }
    fn write(&self,buff:&[u8]) -> Result<(),OpenDP100Error>{
        match self.hid_device.write(&buff){
            Ok(_)=>{
                Ok(())
            }
            Err(_)=>{
                Err(OpenDP100Error::DEVICE)
            }
        }
    }

    fn read(&self,buff:&mut [u8]) -> Result<usize,OpenDP100Error>{
        match self.hid_device.read_timeout(buff,READ_TIME_OUT_MS){
            Ok(size)=>{
                Ok(size)
            }
            Err(_)=>{
                Err(OpenDP100Error::DEVICE)
            }
        }
    }
    
    fn session(&self,request:&Frame) -> Result<Frame,OpenDP100Error>{
        // write request
        let mut output = [0u8;64];
        serialize_out_frame(request, &mut output );
        // print!("Write:");
        // for d in output{
        //     print!("{:02x}",d);
        // }
        // println!();
        self.write(&output)?;


        // read response
        let mut frame = Frame::empty();
        let mut input = [0u8;64];
        self.read(&mut input)?;

        // print!("Read:");
        // for d in input{
        //     print!("{:02x}",d);
        // }
        // println!();

        let res = deserialize_in_frame(&input, &mut frame);
        if res.is_err(){
            return Err(OpenDP100Error::DEVICE);
        }

        Ok(frame)
    }

    pub fn device_info(&self)->Result<DeviceInfo,OpenDP100Error>{
        let req = Frame::new(OpCode::DeviceInfo, &[0u8;0]);
        session_impl!(self,OpCode::DeviceInfo,&req,DeviceInfo)
    }
    
    pub fn basic_info(&self)->Result<BasicInfo,OpenDP100Error>{
        let req = Frame::new(OpCode::BasicInfo, &[0u8;0]);
        session_impl!(self,OpCode::BasicInfo,&req,BasicInfo)
    }

    pub fn sys_info(&self)->Result<SystemInfo,OpenDP100Error>{
        let req = Frame::new(OpCode::SystemInfo, &[0u8;0]);
        session_impl!(self,OpCode::SystemInfo,&req,SystemInfo)
    }

    pub fn basic_set(&self,idx:usize)->Result<BasicSet,OpenDP100Error>{
        if idx>10{
            return Err(OpenDP100Error::INVALID_PARAM);
        }
        let req = Frame::new(OpCode::BasicSet, &[idx as u8;1]);
        session_impl!(self,OpCode::BasicSet,&req,BasicSet)
    }

    pub fn current_basic_set(&self)->Result<BasicSet,OpenDP100Error>{
        let req = Frame::new(OpCode::BasicSet, &[0x80u8;1]);
        session_impl!(self,OpCode::BasicSet,&req,BasicSet)
    }

    pub fn update_basic_set(&self,set_req:&BasicSet,switch:bool)->Result<(),OpenDP100Error>{
        let mut set = (*set_req).clone();
        set.index = {if switch {0xa0} else {0x20}} + set.index;

        let req: Frame = Frame::new(OpCode::BasicSet, &set.to_data());
        let r = session_impl!(self,OpCode::BasicSet,&req,OperationResult);
        
        match r {
            Ok(ok) => {
                match ok.result {
                    data::OpResult::Success=>{
                        Ok(())
                    }
                    _=>{
                        Err(OpenDP100Error::DEVICE)
                    }
                } 
            }
            Err(e)=>{
                Err(e)
            }
        }
    }

}

/** High level api */
impl OpenDP100 {
    
    pub fn set_output_on(&self,on:OutputState) -> Result<(),OpenDP100Error>{
        let mut basic_set = self.current_basic_set()?;
        if basic_set.state == on{
            return Ok(())
        }
        basic_set.state = on;
        self.update_basic_set(&basic_set, false)?;
        Ok(())
    }
    
    pub fn switch_config(&self,idx:usize) -> Result<(),OpenDP100Error>{
        let config_set = self.basic_set(idx)?;
        self.update_basic_set(&config_set, true)?;
        Ok(())
    }

}