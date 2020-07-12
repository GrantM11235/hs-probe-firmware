#![no_std]
#![no_main]

mod app;
mod usb;

use panic_rtt_target as _;
use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let rcc = hs_probe_bsp::rcc::RCC::new(stm32ral::rcc::RCC::take().unwrap());

    let usb_global = stm32ral::otg_fs_global::OTG_FS_GLOBAL::take().unwrap();
    let usb_device = stm32ral::otg_fs_device::OTG_FS_DEVICE::take().unwrap();
    let usb_pwrclk = stm32ral::otg_fs_pwrclk::OTG_FS_PWRCLK::take().unwrap();
    let mut usb = crate::usb::USB::new(usb_global, usb_device, usb_pwrclk);

    // Create App instance with the HAL instances
    let mut app = app::App::new(&rcc, &mut usb);

    rprintln!("Starting...");

    // Initialise application, including system peripherals
    unsafe { app.setup() };

    loop {
        // Process events
        app.poll();
    }
}