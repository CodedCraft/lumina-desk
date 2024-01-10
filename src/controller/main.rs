use anyhow::Result;
use esp_idf_sys::{
    self, esp_deepsleep_gpio_wake_up_mode_t_ESP_GPIO_WAKEUP_GPIO_LOW,
    // gpio_int_type_t_GPIO_INTR_LOW_LEVEL,
};
use std::time::Duration;
use setup::*;

mod setup;

const LIGHT_ADDR: [u8; 6] = [0xA0, 0x76, 0x4E, 0x1B, 0xBB, 0x74];
const STANDBY_INTERVAL: u64 = 10_000_000; // in micro-seconds
// const IDLE_INTERVAL: u64 = 10_000_000; // in micro-seconds
// const ACTIVE_INTERVAL: u64 = 20_000; // in micro-seconds

#[derive(Debug)]
enum LampMode {
    Dimming,
    Color,
    // Standby,
}

// #[link_section = ".rtc.data.rtc_memory"]
// static mut LAMPMODE: LampMode = LampMode::Standby;

fn main() -> Result<()> {
    esp_idf_hal::sys::link_patches();

    let wakeup_cause = get_wakeup_cause();
    let mut devices = setup::setup(wakeup_cause)?;
    let mut lamp_mode = LampMode::Dimming;
    let mut angle = devices
        .position_sensor
        .angle()
        .expect("Could not get angle") as i32;

    unsafe {
        esp_idf_sys::esp_deep_sleep_enable_gpio_wakeup(
            1 << 2,
            esp_deepsleep_gpio_wake_up_mode_t_ESP_GPIO_WAKEUP_GPIO_LOW,
        );
        esp_idf_sys::esp_sleep_enable_gpio_wakeup();
    }

    // let vbat = (devices.vbat.read(&mut devices.vbat_pin)? as f64 * 0.2).round() / 100.0;

    if wakeup_cause == 4 {
        // let msg = format!("Voltage: {vbat}");
        // send_command(&mut devices, msg.as_bytes())?;
        unsafe {
            esp_idf_sys::esp_sleep_enable_timer_wakeup(STANDBY_INTERVAL);
            esp_idf_sys::esp_deep_sleep_start()
        };
    }

    loop {
        devices.button.tick();
        if devices.button.is_clicked() {
            // let vbat = (devices.vbat.read(&mut devices.vbat_pin)? as f64 * 0.2).round() / 100.0;
            // let msg = format!("Click, Current Voltage: {vbat}");
            let msg = format!("0,0");
            send_command(&mut devices, msg.as_bytes())?;
            // send_command(&mut devices, msg.as_bytes())?;
            unsafe { esp_idf_sys::esp_deep_sleep_start() };
        } else if devices.button.is_double_clicked() {
            lamp_mode = match lamp_mode {
                LampMode::Dimming => LampMode::Color,
                LampMode::Color => LampMode::Dimming,
            }
        } else if devices.button.is_triple_clicked() {
        } else if devices.button.held_time().is_some() {
        }
        devices.button.reset();

        if let Ok(new_angle) = devices.position_sensor.angle() {
            let mut diff = new_angle as i32 - angle;
            angle = new_angle as i32;
            match diff {
                _ if diff.abs() < 20 => continue,        // Debounce small movements
                _ if diff < -1000 => diff = diff + 4085, // Clockwise overflow
                _ if diff > 1000 => diff = diff - 4085,  // Counterclockwise overflow
                // _ if diff > 0 => (), // Clockwise i.e. positiv rotation
                // _ if diff < 0 => (), // Counterclockwise i.e. negative rotation
                _ => (),
            }
            let mode;
            match lamp_mode {
                LampMode::Dimming => mode = 1,
                LampMode::Color => mode = 2,
            }
            let msg = format!("{mode},{diff}");
            send_command(&mut devices, msg.as_bytes())?;
        };

        std::thread::sleep(Duration::from_millis(20));
        // unsafe {
        //     esp_idf_sys::esp_sleep_enable_timer_wakeup(ACTIVE_INTERVAL);
        //     esp_idf_sys::esp_light_sleep_start();
        // }

        //     if !lamp_on {
        //         comms.wifi.start()?;
        //         thread::sleep(Duration::from_millis(20));
        //         let msg = "Going to sleep...";
        //         comms.espnow.send(LIGHT_ADDR.clone(), msg.as_bytes())?;
        //         comms.wifi.stop()?;
        //         unsafe {
        //             esp_idf_sys::esp_light_sleep_start();
        //             // let wakeup = esp_idf_sys::esp_sleep_get_wakeup_cause();
        //             // println!("Wake-up cause: {wakeup}");
        //         }
        //         lamp_on = !lamp_on;
        //         comms.wifi.start()?;
        //         thread::sleep(Duration::from_millis(20));
        //         let msg = "Waking up...";
        //         comms.espnow.send(LIGHT_ADDR.clone(), msg.as_bytes())?;
        //         comms.wifi.stop()?;
        //     }
    }
}

fn send_command(devices: &mut Devices, msg: &[u8]) -> Result<()> {
    devices.wifi.start()?;
    devices.espnow.send(LIGHT_ADDR.clone(), msg)?;
    devices.wifi.stop()?;

    Ok(())
}

fn get_wakeup_cause() -> u32 {
    let wakeup_cause;
    unsafe {
        wakeup_cause = esp_idf_sys::esp_sleep_get_wakeup_cause();
    }
    wakeup_cause
}
