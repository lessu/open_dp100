mod frame_out;
mod frame_in;
mod opcode;
// mod lib;
// use lib as open_dp100;
use open_dp100::{OpenDP100};

fn main() {

    let count = OpenDP100::device_count().expect("Get Device failed");
    if count == 0{
        println!("Device Not found");
        return ;
    }
    let api = OpenDP100::new(0).unwrap();
    println!("{:?}",api.sys_info().unwrap());

    println!("{:?}",api.device_info().unwrap());

    // println!("{:?}",api.firm_info().unwrap());

    println!("{:?}",api.basic_info().unwrap());


}