use std::time::Duration;

use rusb::{DeviceHandle, GlobalContext};

const INTERFACE_ID: u8 = 0x01;
const CONFIGURATION_ID: u8 = 0;
const VENDOR_ID: u16 = 0x046D;
const PRODUCT_ID: u16 = 0xC09D;

// Direction enum to represent the direction of the wave effect
#[derive(Clone)]
pub enum Direction {
    Left = 0x01,
    Right = 0x06,
}

#[derive(Debug)]
pub struct Controller {
    inner: DeviceHandle<GlobalContext>,
    timeout: Duration,
}

impl Controller {
    pub fn new_with_timeout(timeout: Duration) -> rusb::Result<Self> {
        let handle = rusb::open_device_with_vid_pid(VENDOR_ID, PRODUCT_ID);
        match handle {
            Some(handle) => {
                handle.set_active_configuration(CONFIGURATION_ID)?;
                Ok(Self {
                    inner: handle,
                    timeout,
                })
            }
            None => Err(rusb::Error::NoDevice),
        }
    }

    pub fn new() -> rusb::Result<Self> {
        Self::new_with_timeout(Duration::from_secs(2))
    }

    // Takes an array of three 8-bit unsigned integers representing RGB color values.
    // Returns a Result type from the rusb crate, which will be Ok(()) if the command was successful, or an Err containing the error if not.
    pub fn set_solid(&self, rgb: [u8; 3]) -> rusb::Result<()> {
        let [red, green, blue] = rgb;
        self.command(
            &[
                0x11, 0xff, 0x0e, 0x1b, 0x00, 0x01, red, green, blue, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            ],
            true,
        )
    }

    pub fn set_breathe(&self, rgb: [u8; 3], rate: u16, brightness: u8) -> rusb::Result<()> {
        let [red, green, blue] = rgb;
        let rate_bytes = rate.to_be_bytes();
        self.command(
            &[
                0x11,
                0xff,
                0x0e,
                0x1b,
                0x00,
                0x04,
                red,
                green,
                blue,
                rate_bytes[0],
                rate_bytes[1],
                0x00,
                brightness,
                0x00,
                0x00,
                0x00,
                0x01,
                0x00,
                0x00,
                0x00,
            ],
            false,
        )
    }

    pub fn set_cycle(&self, rate: u16, brightness: u8) -> rusb::Result<()> {
        let rate_bytes = rate.to_be_bytes();
        self.command(
            &[
                0x11,
                0xff,
                0x0e,
                0x1b,
                0x00,
                0x02,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                rate_bytes[0],
                rate_bytes[1],
                brightness,
                0x00,
                0x00,
                0x01,
                0x00,
                0x00,
                0x00,
            ],
            true,
        )
    }

    pub fn set_triple(&self, colors: [[u8; 3]; 3]) -> rusb::Result<()> {
        self.command(
            &[
                0x11,
                0xff,
                0x12,
                0x1b,
                0x01,
                colors[0][0],
                colors[0][1],
                colors[0][2],
                0x02,
                colors[1][0],
                colors[1][1],
                colors[1][2],
                0x03,
                colors[2][0],
                colors[2][1],
                colors[2][2],
                0x00,
                0x00,
                0x00,
                0x00,
            ],
            false,
        )
    }

    pub fn set_wave(&self, rate: u16, brightness: u8, direction: Direction) -> rusb::Result<()> {
        let rate_bytes = rate.to_be_bytes();
        self.command(
            &[
                0x11,
                0xff,
                0x0e,
                0x1b,
                0x00,
                0x03,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                rate_bytes[0],
                direction as u8,
                brightness,
                rate_bytes[1],
                0x01,
                0x00,
                0x00,
                0x00,
            ],
            true,
        )
    }

    pub fn set_blend(&self, rate: u16, brightness: u8) -> rusb::Result<()> {
        let rate_bytes = rate.to_be_bytes();
        self.command(
            &[
                0x11,
                0xff,
                0x0e,
                0x1b,
                0x00,
                0x06,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                rate_bytes[0],
                rate_bytes[1],
                brightness,
                0x00,
                0x01,
                0x00,
                0x00,
                0x00,
            ],
            true,
        )
    }

    // This function is called before sending a command to the device.
    // It detaches the kernel driver from the interface and claims the interface for the program.
    // This is necessary to ensure that the program has exclusive access to the device.
    fn command_prologue(&self) -> rusb::Result<()> {
        // Detach the kernel driver from the interface.
        // This allows the program to have exclusive access to the device.
        self.inner.detach_kernel_driver(INTERFACE_ID)?;
        // Claim the interface.
        // This tells the operating system that the program is now in control of the device.
        self.inner.claim_interface(INTERFACE_ID)?;
        Ok(())
    }

    // This function is called after a command has been sent to the device.
    // It releases the interface and reattaches the kernel driver.
    // This is necessary to allow other programs to access the device.
    fn command_epilogue(&self) -> rusb::Result<()> {
        // Release the interface.
        // This tells the operating system that the program is no longer in control of the device.
        self.inner.release_interface(INTERFACE_ID)?;
        // Reattach the kernel driver to the interface.
        // This allows other programs to access the device.
        self.inner.attach_kernel_driver(INTERFACE_ID)?;
        Ok(())
    }

    pub fn command(&self, data: &[u8], disable_ls_memory: bool) -> rusb::Result<()> {
        self.command_prologue()?;

        // If the disable_ls_memory flag is true, send a specific command to the device to disable LS memory.
        if disable_ls_memory {
            self.inner
                .write_control(
                    0x21,
                    0x09,
                    0x210,
                    0x01,
                    &[0x10, 0xff, 0x0e, 0x5b, 0x01, 0x03, 0x05],
                    self.timeout,
                )
                .unwrap();
        }

        // Send the command data to the device.
        self.inner
            .write_control(0x21, 0x09, 0x211, 0x01, data, self.timeout)
            .unwrap();

        // Check if the first four bytes of the command data matches a specific sequence.
        // If it does, send an additional command to the device in order to apply the command (only used when sending `triple`).
        let is_triple_command = data[0..4] == [0x11, 0xff, 0x12, 0x1b];
        if is_triple_command {
            self.inner.write_control(
                0x21,
                0x09,
                0x211,
                0x01,
                &[
                    0x11, 0xff, 0x12, 0x7b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
                self.timeout,
            )?;
        }

        self.command_epilogue()?;
        Ok(())
    }
}
