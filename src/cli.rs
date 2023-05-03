use clap::{Command, Arg, arg, value_parser};
use open_dp100::{OpenDP100, BasicInfo, BasicSet, OutputState, SystemInfo,};

#[derive(Debug)]
struct Config {
    config: Option<u32>,
    on: bool,
    off: bool,
    vout: Option<f32>,
    iout: Option<f32>,
    ovp: Option<f32>,
    ocp: Option<f32>,
}


// Define a trait for printing the structures
pub trait Printable {
    fn print(&self);
}

// Implement the trait for each structure
impl Printable for BasicInfo {
    fn print(&self) {
        println!("Basic Info:");
        println!("  vin:{}V", self.vin as f32 / 1000.0); // structure unit 1mV
        println!("  vout:{}V", self.vout as f32 / 1000.0); // structure unit 1mV
        println!("  iout:{}A", self.iout as f32 / 1000.0); // structure unit mA
        println!("  vo_max:{}V", self.vo_max as f32 / 1000.0); // structure unit 1mV
        println!("  temp1:{}℃", self.temp1 as f32 / 10.0); // structure unit 100℃
        println!("  temp2:{}℃", self.temp2 as f32 / 10.0); // structure unit 100℃
        println!("  dc_5v:{}V", self.dc_5v as f32 / 1000.0); // structure unit 1mV
        println!("  out_mode:{}", self.out_mode);
        println!("  work_st:{}", self.work_st);
        println!();
    }
}

impl Printable for BasicSet {
    fn print(&self) {
        print!("Basic Set <{}>: ",self.index);
        match self.state {
            OutputState::On => println!("On"),
            OutputState::Off => println!("Off"),
        }

        println!("  vo_set:{}V", self.vo_set as f32 / 1000.0); // unit 1mV
        println!("  io_set:{}A", self.io_set as f32 / 1000.0); // unit mA
        println!("  ovp_set:{}V", self.ovp_set as f32 / 1000.0); // unit 1mV
        println!("  ocp_set:{}A", self.ocp_set as f32 / 1000.0); // unit mA
        println!();
    }
}

impl Printable for SystemInfo {
    fn print(&self) {
        println!("System Info:");
        println!("  blk_lev:{}", self.blk_lev);
        println!("  opp:{}W", self.opp as f32 / 100.0); // over protection power unit 100mW
        println!("  opt:{}℃", self.opt as f32 / 10.0); // over protection temperature unit 100m℃
        println!("  vol_kev:{}", self.vol_kev);
        println!();
    }
}

fn main() {
    let matches = Command::new("dp100")
        .version("1.0")
        .author("lessu")
        .about("dp100 cli program")
        .subcommand(
            Command::new("ls")
                .about("print devices that avaliable")
        )
        .subcommand(
            Command::new("status")
                .about("print device status")
                .args(&[
                    arg!(device: -d --device <DEVICE> "select current device").value_parser(value_parser!(u8)).default_value("0"),
                    arg!(allconfig: -a --"all-config" "print all config,if -a is not set,only the config current in use is printed"),
                    arg!(system: -s --system "print system info(eg backlight level...)")
                ]),
        )
        .subcommand(
            Command::new("set")
                .about("set value ")
                .arg(
                    arg!(device: -d --device <DEVICE> "select current device").value_parser(value_parser!(u8)).default_value("0"),
                )
                .arg(
                    arg!([keyvalue] ...  "config=<index>:switch to config before set setting,range 0~9\n\
                                          on:set output on\n\
                                          off:set output off\n\
                                          v=<volt>:set vout,range 0.00~36.00\n\
                                          i=<current>: set iout,range 0.00~10.00\n\
                                          ov=<volt>: set ovp,range 0.00~36.00\n\
                                          oc=<current>: set ocp,range 0.00~10.00")
                )
                .after_help("example:\n\
                                dp100 set on\n\
                                dp100 set config=2\n\
                                dp100 set config=2 on vout=13.4\n\
                            ")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("ls", _ls_matches)) => {
            // Device count:%d
            //    0 <name> sn:<sn> hdw_ver:<hdw_ver> app_ver:<app_ver> <YYYY-mm-dd>
            //    1 <name> sn:<sn> hdw_ver:<hdw_ver> app_ver:<app_ver> <YYYY-mm-dd>
            let count = OpenDP100::device_count().unwrap();
            println!("Device count: {}", count);
            for i in 0..count {
                let device = OpenDP100::new(i).unwrap();
                let info = device.device_info().unwrap();
                let dev_type = String::from_utf8_lossy(&info.dev_type).trim_end_matches(|c:char| c == '\0' || !c.is_ascii()).to_string();
                let dev_sn = info.dev_sn[8..].iter().map(|&x| format!("{:02X}", x)).collect::<Vec<String>>().join("");
                println!(
                    "{} {} sn:{} hdw_ver:{}.{} app_ver:{}.{} {:04}-{:02}-{:02}",
                    i + 1,
                    dev_type,
                    dev_sn,
                    info.hdw_ver/10,info.hdw_ver%10,
                    info.app_ver/10,info.app_ver%10,
                    info.year,
                    info.moon,
                    info.day
                );
            }

        }
        Some(("status", status_matches)) => {
            let device_index:u8 = *status_matches.get_one("device").expect("device setting failed");
            
            let device = OpenDP100::new(device_index as usize).expect("open device failed");
            
            let info = device.device_info().unwrap();

            let dev_type = String::from_utf8_lossy(&info.dev_type).trim_end_matches(|c:char| c == '\0' || !c.is_ascii()).to_string();
            println!("Device {} name:{}",device_index,dev_type);
            device.basic_info().unwrap().print();
            
            if status_matches.get_flag("system") {
                device.sys_info().unwrap().print();
            }

            let current_config = device.current_basic_set().unwrap();
            if status_matches.get_flag("allconfig") {
                for i in 0..9{
                    if current_config.index == i{
                        print!("[*] ",);
                    }else{
                        print!("[ ] ",);
                    }
                    device.basic_set(i as usize).unwrap().print();
                }
            } else {
                current_config.print();
            }


        }
        Some(("set", set_matches)) => {
            let mut config = Config {
              config: None,
              on: false,
              off: false,
              vout: None,
              iout: None,
              ovp: None,
              ocp: None,
            };
            let device_index:u8 = *set_matches.get_one("device").expect("device setting failed");
   
            let device = OpenDP100::new(device_index as usize).expect("open device failed");
    
            let keyvalues:Vec<&String> = set_matches.get_many("keyvalue")
                .expect("at least on param should be set")
                .collect();

            for keyvalue in keyvalues.iter() {
              let kv: Vec<&str> = keyvalue.split("=").collect();
              match kv[0] {
                  "config" => {
                      let index = kv[1].parse::<u32>().unwrap();
                      if index > 9 {
                          panic!("config index out of range");
                      }
                      config.config = Some(index);
                  },
                  "on" => config.on = true,
                  "off" => config.off = true,
                  "v" => {
                      let volt = kv[1].parse::<f32>().unwrap();
                      if volt < 0.0 || volt > 36.0 {
                          panic!("vout out of range");
                      }
                      config.vout = Some(volt);
                  },
                  "i" => {
                      let current = kv[1].parse::<f32>().unwrap();
                      if current < 0.0 || current > 36.0 {
                          panic!("iout out of range");
                      }
                      config.iout = Some(current);
                  },
                  "ov" => {
                      let volt = kv[1].parse::<f32>().unwrap();
                      if volt < 0.0 || volt > 36.0 {
                          panic!("ovp out of range");
                      }
                      config.ovp = Some(volt);
                  },
                  "oc" => {
                      let current = kv[1].parse::<f32>().unwrap();
                      if current < 0.0 || current > 36.0 {
                          panic!("ocp out of range");
                      }
                      config.ocp = Some(current);
                  },
                  _ => panic!("Invalid key-value pair"),
              }
            }

            if let Some(idx) = config.config{
                let current_set = device.current_basic_set().unwrap();
                if current_set.index != idx as u8 {
                    device.switch_config(idx as usize).unwrap();
                }
            }
            let mut current_set = device.current_basic_set().unwrap();

            if config.on {
                current_set.state = OutputState::On;
            }
            if config.off {
                current_set.state = OutputState::Off;
            }
            if let Some(vout) = config.vout{
                current_set.vo_set = (vout * 1000.0) as u16;
            }
            if let Some(iout) = config.iout{
                current_set.io_set = (iout * 1000.0) as u16;
            }            
            if let Some(ovp) = config.ovp{
                current_set.ovp_set = (ovp * 1000.0) as u16;
            }
            if let Some(ocp) = config.ocp{
                current_set.ocp_set = (ocp * 1000.0) as u16;
            }
            device.update_basic_set(&current_set, false).unwrap();

        }
        _ => unreachable!(),
    }
}