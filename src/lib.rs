use frame_out::{BasicInfoOut, serialize_out_frame, OperationOut,FirmInfoOut, DeviceInfoOut, SystemInfoOut};
use frame_in::{BasicInfo, OperationIn, deserialize_in_frame, InFrame, FirmInfo, SystemInfo,  DeviceInfo};
use hidapi::{HidApi, HidDevice};
use error::OpenDP100Error;
use opcode::OpCode;

mod frame_out;
mod frame_in;
mod opcode;
mod error;

const VID: u16 = 0x2e3c;
const PID: u16 = 0xaf01;
const READ_TIME_OUT_MS : i32 =200;

pub struct OpenDP100{
    hid_device:HidDevice,
}

macro_rules! create_req_impl {
    ($fn_name:tt,$op_code:pat,$req:ident,$res:ident) => {
        pub fn $fn_name(&self)->Result<$res,OpenDP100Error>{
            let mut retry = 0;
            let response_res = loop{
                let frame = self.session(&$req{ })?;

                match frame.op_code {
                    $op_code => {
                        break $res::from_data(frame.data());
                    }
                    _ => {
                        if retry == 3{
                            return Err(OpenDP100Error::DEVICE);
                        }else{
                            retry = retry + 1;
                            continue;
                        }
                    }
                }
            };

            match response_res{
                Ok(response) => {
                    return Ok(response);
                }
                Err(_)=>{
                    return Err(OpenDP100Error::DEVICE);
                }
            }
        }
    };
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
    
    fn session(&self,request:&dyn OperationOut) -> Result<InFrame,OpenDP100Error>{
        // write request
        let mut output = [0u8;64];
        serialize_out_frame(request, &mut output );
        print!("Write:");
        for d in output{
            print!("{:02x}",d);
        }
        println!();
        self.write(&output)?;


        // read response
        let mut frame = InFrame::empty();
        let mut input = [0u8;64];
        self.read(&mut input)?;

        print!("Read:");
        for d in input{
            print!("{:02x}",d);
        }
        println!();

        let res = deserialize_in_frame(&input, &mut frame);
        if res.is_err(){
            return Err(OpenDP100Error::DEVICE);
        }

        Ok(frame)
    }

    create_req_impl!(device_info,OpCode::DeviceInfo,DeviceInfoOut,DeviceInfo);
    // create_req_impl!(firm_info,OpCode::FirmInfo,FirmInfoOut,FirmInfo);
    create_req_impl!(basic_info,OpCode::BasicInfo,BasicInfoOut,BasicInfo);
    create_req_impl!(sys_info,OpCode::SystemInfo,SystemInfoOut,SystemInfo);

}