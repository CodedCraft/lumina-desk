use std::{thread, time::Duration};
use anyhow::Result;
use button_driver::{Button, ButtonConfig};
use as5600::As5600;
use esp_idf_svc::{espnow::*, wifi::*};
use esp_idf_hal::{i2c::*, adc::*, gpio::*};
use crate::LIGHT_ADDR;

pub struct Devices {
    pub position_sensor: As5600<I2cDriver<'static>>,
    pub button: Button<PinDriver<'static, Gpio2, Input>>,
    pub vbat: AdcDriver<'static, ADC1>,
    pub vbat_pin: AdcChannelDriver<'static, 3, Gpio3>,
    pub wifi: EspWifi<'static>,
    pub espnow: EspNow<'static>,
}

pub fn setup(wakeup_cause: u32) -> Result<Devices> {
    let peripherals = esp_idf_hal::peripherals::Peripherals::take()?;

    // Configure Wifi
    let modem = peripherals.modem;
    let sys_loop = esp_idf_svc::eventloop::EspSystemEventLoop::take()?;
    let nvs = esp_idf_svc::nvs::EspDefaultNvsPartition::take()?;
    let mut wifi = esp_idf_svc::wifi::EspWifi::new(modem, sys_loop.clone(), Some(nvs))?;

    wifi.set_configuration(&esp_idf_svc::wifi::Configuration::Client(
        esp_idf_svc::wifi::ClientConfiguration::default(),
    ))?;
    // thread::sleep(Duration::from_millis(100));
    wifi.start()?;
    // unsafe {
    //     esp_idf_sys::esp_wifi_set_ps(esp_idf_sys::wifi_ps_type_t_WIFI_PS_MAX_MODEM);
    // }

    // Configure Esp-Now
    let espnow = EspNow::take()?;
    // espnow.register_recv_cb(move |_addr, msg| {
    //     println!(
    //         "Message: {}",
    //         std::str::from_utf8(msg).expect("Could not convert message to str")
    //     );
    // })?;
    let peer_info = PeerInfo {
        peer_addr: LIGHT_ADDR.clone(),
        ifidx: WifiDeviceId::Sta.into(),
        channel: 0,
        encrypt: false,
        lmk: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ..Default::default()
    };

    espnow.add_peer(peer_info)?;
    if wakeup_cause != 4 {
        // espnow.send(LIGHT_ADDR.clone(), &[1])?;
        let msg = format!("1,1000");
        espnow.send(LIGHT_ADDR.clone(), msg.as_bytes())?;
    }
    wifi.stop()?;

    // Configure AS5600
    let i2c_pin = peripherals.i2c0;
    let sda = peripherals.pins.gpio4;
    let scl = peripherals.pins.gpio5;
    let i2c_config = I2cConfig::new();
    let i2c = I2cDriver::new(i2c_pin, sda, scl, &i2c_config)?;
    let mut position_sensor = As5600::new(i2c);
    let as5600_config = as5600::configuration::Configuration {
        power_mode: as5600::configuration::PowerMode::Lpm1,
        hysteresis: as5600::configuration::Hysteresis::Lsb3,
        output_stage: as5600::configuration::OutputStage::Analog,
        pwm_frequency: as5600::configuration::PwmFreq::PwmF1,
        slow_filter: as5600::configuration::SlowFilterMode::X16,
        fast_filter_threshold: as5600::configuration::FastFilterThreshold::SlowFilterOnly,
        watchdog_state: as5600::configuration::WatchdogState::On,
    };
    position_sensor
        .set_config(as5600_config)
        .expect("Error configuring the As5600");

    // Configure Button
    let button_pin = esp_idf_hal::gpio::PinDriver::input(peripherals.pins.gpio2)?;
    while button_pin.is_low() {
        // Debounce a long press by the user
        thread::sleep(Duration::from_millis(10));
    }
    let button = Button::new(button_pin, ButtonConfig::default());

    // Configure Voltage Measurment via ADC
    let vbat = AdcDriver::new(
        peripherals.adc1,
        &esp_idf_hal::adc::config::Config::new().calibration(true),
    )?;
    let vbat_pin: AdcChannelDriver<{ esp_idf_hal::adc::attenuation::DB_11 }, _> =
        AdcChannelDriver::new(peripherals.pins.gpio3)?;
    // Power Management Settings
    let pm_config = esp_idf_sys::esp_pm_config_esp32c3_t {
        max_freq_mhz: 80,
        min_freq_mhz: 40,
        light_sleep_enable: true,
    };
    unsafe {
        esp_idf_sys::esp_pm_configure(&pm_config as *const _ as *const _);
    }

    let devices = Devices {
        position_sensor,
        button,
        vbat,
        vbat_pin,
        wifi,
        espnow,
    };

    Ok(devices)
}
