use rusb::{Context, Device, DeviceDescriptor, DeviceHandle, UsbContext, UsbOption};
use std::thread::sleep;
use std::time::Duration;

const VID: u16 = 0x0E8D;
const PID: u16 = 0x2000;
const CONFIGURATION_VALUE: u8 = 1;
const INTERFACE_INDEX: u8 = 0;
const BULK_WRITE_ENDPOINT: u8 = 0x01; // The address of the bulk endpoint for writing
const BULK_READ_ENDPOINT: u8 = 0x81; // The address of the bulk endpoint for reading
const WRITE_TIMEOUT: Duration = Duration::from_secs(1); // Timeout for the write operation
const READ_TIMEOUT: Duration = Duration::from_secs(1); // Timeout for the read operation

fn main() -> rusb::Result<()> {
    let ctx = Context::with_options(&[UsbOption::use_usbdk()])?;

    loop {
        let devs = ctx.devices()?;
        let mut found = false;

        for dev in devs.iter() {
            match dev.device_descriptor() {
                Ok(desc) => {
                    if desc.vendor_id() == VID && desc.product_id() == PID {
                        println!(
                            "Found Device With VID 0x{:04X} And PID 0x{:04X}. Device Object: {:?}, Device Descriptor: {:?}",
                            VID, PID, dev, desc
                        );

                        found = true;
                        match dev.config_descriptor(1) {
                            Ok(config_desc) => {
                                println!("Configuration Descriptor: {:?}", config_desc);
                            }
                            Err(e) => {
                                eprint!("Failed to get configuration descriptor: {}", e);
                            }
                        }
                        match dev.open() {
                            Ok(mut handle) => {
                                match handle.set_active_configuration(CONFIGURATION_VALUE) {
                                    Ok(()) => {
                                        println!(
                                            "Configuration {} set successfully.",
                                            CONFIGURATION_VALUE
                                        );
                                    }
                                    Err(e) => {
                                        eprint!("Failed to set configuration: {}", e);
                                        continue;
                                    }
                                }

                                match handle.claim_interface(INTERFACE_INDEX) {
                                    Ok(()) => {
                                        println!(
                                            "Interface {} claimed successfully.",
                                            INTERFACE_INDEX
                                        );
                                    }
                                    Err(e) => {
                                        eprint!("Failed to claim interface: {}", e);
                                    }
                                }

                                let data: [u8; 1] = [0xA0];
                                match handle.write_bulk(BULK_WRITE_ENDPOINT, &data, WRITE_TIMEOUT) {
                                    Ok(bytes_written) => {
                                        println!(
                                            "Wrote {} bytes to endpoint 0x{:02X}.",
                                            bytes_written, BULK_WRITE_ENDPOINT
                                        );
                                    }
                                    Err(e) => {
                                        eprint!("Failed to write to bulk endpoint: {}", e);
                                    }
                                }

                                let mut buf = [0u8; 1];
                                match handle.read_bulk(BULK_READ_ENDPOINT, &mut buf, READ_TIMEOUT) {
                                    Ok(bytes_read) => {
                                        println!(
                                            "Read {} bytes from endpoint 0x{:02X}. Data: {:?}",
                                            bytes_read,
                                            BULK_READ_ENDPOINT,
                                            &buf[..bytes_read]
                                        );
                                    }
                                    Err(e) => {
                                        eprint!("Failed to read from bulk endpoint: {}", e);
                                    }
                                }

                                let mut buf = [0u8; 4];
                                match handle.read_bulk(BULK_READ_ENDPOINT, &mut buf, READ_TIMEOUT) {
                                    Ok(bytes_read) => {
                                        println!(
                                            "Read {} bytes from endpoint 0x{:02X}. Data: {:?}",
                                            bytes_read,
                                            BULK_READ_ENDPOINT,
                                            &buf[..bytes_read]
                                        );
                                    }
                                    Err(e) => {
                                        eprint!("Failed to read from bulk endpoint: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                eprint!("Failed to open device: {}", e);
                            }
                        }

                        break;
                    }
                }
                Err(e) => {
                    eprint!("Failed to get device descriptor: {}", e);
                }
            }
        }

        if found {
            println!(
                "Device found, configuration set, interface claimed, data written, and data read."
            );
            break;
        } else {
            println!(
                "Device With VID 0x{:04X} And PID 0x{:04X} Not Found. Retrying...",
                VID, PID
            );
        }

        sleep(Duration::from_millis(500));
    }

    Ok(())
}
