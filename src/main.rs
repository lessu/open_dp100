use std::time::Duration;
use open_dp100::{OpenDP100, OutputState};


fn main() {

    let count = OpenDP100::device_count().expect("Get Device failed");
    if count == 0{
        println!("Device Not found");
        return ;
    }
    let api = OpenDP100::new(0).unwrap();
    println!("{:?}",api.sys_info().unwrap());

    println!("{:?}",api.device_info().unwrap());

    println!("{:?}",api.basic_info().unwrap());

    println!("{:?}",api.current_basic_set().unwrap());

    api.switch_config(6).unwrap();

    api.set_output_on(OutputState::On).unwrap();
    std::thread::sleep(Duration::from_secs(3));
    api.set_output_on(OutputState::Off).unwrap();

}