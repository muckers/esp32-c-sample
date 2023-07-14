use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use log::*;
use std::thread;
use std::time::Duration;

use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, wifi::EspWifi};

fn test_https_client() -> anyhow::Result<(), anyhow::Error> {
    use embedded_svc::http::client::*;
    use embedded_svc::utils::io;
    use esp_idf_svc::http::client::*;

    let url = String::from("https://slashdot.org");

    info!("About to fetch content from {}", url);

    let mut client = Client::wrap(EspHttpConnection::new(&Configuration {
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),

        ..Default::default()
    })?);

    let mut response = client.get(&url)?.submit()?;

    let mut body = [0_u8; 3048];

    let read = io::try_read_full(&mut response, &mut body).map_err(|err| err.0)?;

    info!(
        "Body (truncated to 3K):\n{:?}",
        String::from_utf8_lossy(&body[..read]).into_owned()
    );

    // Complete the response
    while response.read(&mut body)? > 0 {}

    Ok(())
}
fn main() -> Result<(), anyhow::Error> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi_driver = EspWifi::new(peripherals.modem, sys_loop, Some(nvs)).unwrap();

    wifi_driver
        .set_configuration(&Configuration::Client(ClientConfiguration {
            ssid: "CountrySide".into(),
            password: "%Burn1n4t1ng%".into(),
            ..Default::default()
        }))
        .unwrap();

    wifi_driver.start().unwrap();
    wifi_driver.connect().unwrap();
    while !wifi_driver.is_connected().unwrap() {
        let config = wifi_driver.get_configuration().unwrap();
        println!("Waiting for station {:?}", config);
        thread::sleep(Duration::from_millis(1000));
    }

    println!("Should be connected now");
    // loop {
    println!(
        "IP info: {:?}",
        wifi_driver.sta_netif().get_ip_info().unwrap()
    );
    thread::sleep(Duration::new(10, 0));
    // }

    let _ = test_https_client();

    // let peripherals = Peripherals::take().unwrap();
    let pin = peripherals.pins.gpio4;

    // esp-idf-hal >= "0.39.3"
    let mut led = PinDriver::output(pin)?;

    loop {
        led.toggle()?;

        // we are using thread::sleep here to make sure the watchdog isn't triggered
        thread::sleep(Duration::from_millis(1100));

        led.set_low()?;
        thread::sleep(Duration::from_millis(500));

        // info!("Hello, world!");
    }
}
