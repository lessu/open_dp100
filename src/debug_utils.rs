use rusb::{Device, Direction, TransferType};


pub fn print_device_info(device: &Device<rusb::GlobalContext>) {
    let device_desc = match device.device_descriptor() {
        Ok(desc) => desc,
        Err(_) => return,
    };

    println!(
        "Bus {:03} Device {:03} ID {:04x}:{:04x}",
        device.bus_number(),
        device.address(),
        device_desc.vendor_id(),
        device_desc.product_id()
    );

    for n in 0..device_desc.num_configurations() {
        let config_desc = match device.config_descriptor(n) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                println!(
                    "  Configure {}, Interface Num {}, Setting Num {}",
                    config_desc.number(),
                    interface_desc.interface_number(),
                    interface_desc.setting_number()
                );
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    let direction = match endpoint_desc.direction() {
                        Direction::In => "IN",
                        Direction::Out => "OUT",
                    };
                    let transfer_type = match endpoint_desc.transfer_type() {
                        TransferType::Control => "Control",
                        TransferType::Isochronous => "Isochronous",
                        TransferType::Bulk => "Bulk",
                        TransferType::Interrupt => "Interrupt",
                    };
                    println!(
                        "    Endpoint: Address: {:02x}, Direction: {}, Transfer Type: {}",
                        endpoint_desc.address(),
                        direction,
                        transfer_type
                    );
                }
            }
        }
    }
}
