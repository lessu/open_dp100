use hidapi::HidApi;

use crate::{frame_out::{serialize_out_frame, BasicInfoOut}, frame_in::{deserialize_in_frame, InFrame, BasicInfo, OperationIn}};
mod frame_out;
mod frame_in;
mod opcode;

const VID: u16 = 0x2e3c;
const PID: u16 = 0xaf01;

fn main() {
    println!("Printing all available hid devices:");

    let r = HidApi::new();
    if r.is_err() {
        eprintln!("Error: {}", r.err().unwrap());
        return ;
    }

    let api = r.ok().unwrap();
    
    let mut device_info = None;

    for device in api.device_list() {
        if device.vendor_id() == VID && device.product_id() == PID{
            device_info = Some(device.clone());
        }
    }

    if device_info.is_none() {
        eprintln!("Device Not found");
        return ;
    }

    let device_info = device_info.unwrap();

    println!(
        "Opening device:\n VID: {:04x}, PID: {:04x}\n",
        device_info.vendor_id(),
        device_info.product_id()
    );

    let device = device_info.open_device(&api).unwrap();
    let mut input_buffer = [0u8; 64];

    let mut output_buffer = [0u8; 64];

    let op = BasicInfoOut{};
    serialize_out_frame(&op, &mut output_buffer);
    println!("Write {:?}", &output_buffer);

    device.write(&output_buffer).unwrap();


    let mut frame: InFrame = InFrame::empty();
    loop {
        let len = device.read(&mut input_buffer).unwrap();
        
        match deserialize_in_frame(&input_buffer, &mut frame) {
            Ok(())=>{
                println!("OpCode: {:?}", frame.op_code);
                println!("len: {:?}", frame.op_data_len);
                for d in frame.data(){
                    print!("{:02x} ",d);
                }
                println!();

                match frame.op_code {
                    opcode::OpCode::BasicInfo=>{
                        match BasicInfo::from_data(frame.data()){
                            Ok(info)=>{
                                println!("{:0?} ",info);
                            }
                            Err(())=>{
                                println!("Parse Basic Info Err");
                            }
                        };
                    }
                    _=>{

                    }
                }

            }
            Err(e)=>{
                println!("deseriialize error {:?}",e);
            }
        }
    }

}