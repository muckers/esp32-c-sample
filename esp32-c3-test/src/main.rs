use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use log::*;
use std::thread;
use std::time::Duration;

use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_idf_hal::gpio::Gpio1;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::gpio::Pins;
use esp_idf_hal::modem::Modem;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, wifi::EspWifi};

// TODO: Set these appropriate for your desired wifi network
//
static WIFI_SSID: &str = "CountrySide";
static WIFI_PW: &str = "%Burn1n4t1ng%";

// Using our WiFi connection, make a [GET] request to the URL below and output the first 3K
// of the response.
//
fn make_https_request() {
    use embedded_svc::http::client::*;
    use embedded_svc::utils::io;
    use esp_idf_svc::http::client::*;

    static URL: &str = "https://slashdot.org";
    info!("About to fetch content from {}", URL);

    let mut client = Client::wrap(
        EspHttpConnection::new(&Configuration {
            crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
            ..Default::default()
        })
        .expect("Failed to create http client"),
    );

    let mut response = client
        .get(URL)
        .expect("Failed to get valid URL")
        .submit()
        .expect("Failed to get http response");

    let mut body = [0_u8; 3048];
    let read = io::try_read_full(&mut response, &mut body)
        .map_err(|err| err.0)
        .expect("Failed to read full response from server");

    info!(
        "Body (truncated to 3K):\n{:?}",
        String::from_utf8_lossy(&body[..read]).into_owned()
    );

    // Complete the response
    while response
        .read(&mut body)
        .expect("Failed to read remaining response")
        > 0
    {}
}

// Connect to a WiFi network who's credentials are in WIFI_SSID and WIFI_PW. Note
// this method retries indefinitely until it connects, so check the console output
// if you're not seeing the connect made
//
fn connect_to_wifi(modem: Modem) -> EspWifi<'static> {
    //
    let sys_loop = EspSystemEventLoop::take().expect("Failed to get event loop");
    let nvs = EspDefaultNvsPartition::take().expect("Failed to get nvs partition");

    let mut wifi_driver =
        EspWifi::new(modem, sys_loop, Some(nvs)).expect("Failed to get wifi_driver");

    wifi_driver
        .set_configuration(&Configuration::Client(ClientConfiguration {
            ssid: WIFI_SSID.into(),
            password: WIFI_PW.into(),
            ..Default::default()
        }))
        .expect("Failed to configure the WiFi driver");

    wifi_driver.start().expect("Failed to start WiFi driver");
    wifi_driver
        .connect()
        .expect("Failed to connect operation on WiFi driver");

    while !wifi_driver
        .is_connected()
        .expect("Something failed during connection attempt")
    {
        let config = wifi_driver
            .get_configuration()
            .expect("Failed to get the configuration for the wifi driver");

        info!("Waiting for station {:?}", config);
        thread::sleep(Duration::from_millis(1000));
    }

    info!("Should be connected now");
    info!(
        "IP info: {:?}",
        wifi_driver
            .sta_netif()
            .get_ip_info()
            .expect("Failed to get the IP address for our connection")
    );

    // Just pause for effect, i.e. to read the console output for errors, etc.
    //
    thread::sleep(Duration::from_secs(10));

    wifi_driver // return with static lifetime so WiFi connection stays active even after return
}

fn main() {
    //
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    // TODO: Peripherals is an odd type of singleton -- as far as I can tell, you can only get it once. Is there a way to get it more than once?
    let peripherals = Peripherals::take().expect("failed to get the available peripherals");

    // Attempt to connect to WiFi -- will not return until it
    // successfully connects -- we capture the return value so
    // the compiler doesn't drop it and sever the wifi connection
    //
    let _wifi: EspWifi<'static> = connect_to_wifi(peripherals.modem);

    // Make a basic https request to ensure WiFi connection is working
    //
    make_https_request();

    // This method never returns, loops forever blinking an LED on GPIO 4 (for esp32-C6) or GPIO1 (for esp32-c3)
    //
    let pins = peripherals.pins;
    blink_led(pins);
}

// Loop forever blinking the appropriate LED
//
fn blink_led(pins: Pins) {
    let pin: Gpio1 = pins.gpio1;
    let mut led = PinDriver::output(pin).expect("failed to get LED pin reference");
    loop {
        led.toggle().expect("Failed to toggle LED state");
        thread::sleep(Duration::from_millis(500));
    }
}
