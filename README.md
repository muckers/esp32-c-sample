
## Rust on ESP32-c3/6 with STD, Simplified

There are lots of great references for people wanting to use Rust on the extensa ESP32-c? series of micro-controllers, but even so, I've struggled for a few weeks to find any one reference that just worked out of the box (based on the instructions). Being newish to Rust, and even newer to Rust in embedded programming (I have solid Arduino knowledge) didn't enhance my success.

However, recently I believe I've stumbled on the most straightforward set of steps necessary to do ESP-c? development in Rust, on a modern MacOS machine. This project attempts to lay those steps out in the hope that someone else finds it useful. Stay tuned to the end of these instructions for ideas on how it could be even better (and PR's helping will be much appreciated).

### Prerequisites

These instructions assume the following:

* You're using a relatively recent version of Rust and have the nightly toolchain installed. At the time of this writing I am using v1.73.0-nightly.
* You're on relatively modern version MacOS. I am on 13.4 (Ventura). These instructions may work on other operating systems, e.g. Linux, but will almost certainly need some adjustments. See requests for PR's to make this better.
* Clone this repos!
* A working ESP32-C3 or ESP32-C6 micro-controller, a breadboard (and jumpers), an LED, and a resistor around 1K

### Install dependencies

You'll need `espflash` installed:

```
cargo install espflash
```

You'll need `espup` installed:

```
cargo install espup
```

Once that finishes installing, use it to install the extensa Rust toolchain/support:

```
cd ~
espup install
```

In addition to installing all the necessary Rust support, it will create the file `~/export-esp.sh` for you. Source this ( `. ~/export-esp.sh` ) to get the environment all setup, or add it to your shell's startup if you're going to be doing this a lot. I've tested this on both `bash` and `zsh`.

Now you're ready to try and build one of the samples in this repos. But before doing so, edit `main.rs` and change the `WIFI_SSID` and `WIFI_PW` constants to match your network, then save the file and do the following:

```
cd esp32-c3-test
cargo update
cargo build
```

If you don't get a successful build out of this step, trace your steps and make sure you have all the dependencies above accounted for.

Now, If you have, in this case, a working ESP32-C3 (e.g., [this one from Adafruit](https://www.adafruit.com/product/5337) ), wired up in a breadboard configured as pictureed, you're ready to attempt to load your code onto the micro-controller and see it work:

```
cargo run
```

At which point you'll see `espflash` get run and it will ask you for the [USB] serial port the device is on. Select the appropriate port (for me, the first one in the list is always right), and hit enter. Once the code is uploaded and starts running, the sample does the following:

1. Connects to the WiFi network you configured above
2. Connects to https://slashdot.org and does a GET request for the home page, outputting the first 3K of it to the console
3. Drops into an infinite loop, blinking an LED on GPIO 1 (on the C3), or GPIO 4 (on the C6).

Do note that if you have the ESP32-C6 and are building that sample, the breadboard configuration is slightly different.

## Requests for help

As I said, I am using this project to continue my Rust journey, which I am early into. I have a garden project running on an ESP8266 using Arduino (C) code, and would like to convert it over to Rust on the ESP32-C3. 

All that is to say, I have a very basic understanding, and would love more expert help on this sample project in the following areas:

1. The code for the C3 and C6 samples are identical except for the GPIO pin used for the LED. I would love to collapse the sample into a single code base, since other than the LED the only differences are build flag for the two different processors. I am not sure how to accommodate this myself.
2. Test these instructions on Linux (or Windows) and help me update with any differences so more people on more platforms can use this for ESP32 development.
3. Expand the samples to do other things. Both the C3 and C6 have an addressable RGB LED that I am not sure how to control. Would be cool to expand the samples to demo this LED.

