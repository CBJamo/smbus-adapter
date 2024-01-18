# I2c to Smbus Adapter

Smbus is mostly compatable with i2c. This tiny crate does not attempt to address
differences such as transfer timeouts, the alert line, or error checking.
It simply provides helper functions for the smbus standardized transfers.

Note that the smbus spec only allows 100kHz clock speed.
If operating a mixed i2c and smbus bus, and you need to 
communicate with the i2c devices at more than 100kHz, I recomend using the [I2cDeviceWithConfig](https://docs.embassy.dev/embassy-embedded-hal/git/default/shared_bus/asynch/i2c/struct.I2cDeviceWithConfig.html)
struct provided by [embassy-embedded-hal](https://crates.io/crates/embassy-embedded-hal).

The SmbusAdapter struct accepts anything that impls the [I2c](https://docs.rs/embedded-hal-async/latest/embedded_hal_async/i2c/trait.I2c.html) trait.
It also impls the I2c trait on itself, for devices that use both standard Smbus commands and direct register access.

Currently, only async operation is supported.
