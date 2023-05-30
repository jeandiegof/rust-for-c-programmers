#![allow(dead_code)]

struct GpioConfig<ENABLED, DIRECTION, PIN> {
    pin: PIN,
    enabled: ENABLED,
    direction: DIRECTION,
}

// ENABLED
struct Enabled;
struct Disabled;

// DIRECTION
struct Output;
struct Input;
struct DontCare;

// PINS
#[derive(Debug)]
struct PORTA_0;

#[derive(Debug)]
struct PORTA_1;

#[derive(Debug)]
struct PORTB_0;

#[derive(Debug)]
struct PORTB_1;

// functions available for all pins
impl<ENABLED, DIRECTION, PIN> GpioConfig<ENABLED, DIRECTION, PIN> {
    pub fn new(pin: PIN) -> GpioConfig<Disabled, DontCare, PIN> {
        // call you hardware api here to mark the pin as disabled/high-z/whatever

        GpioConfig {
            pin,
            enabled: Disabled,
            direction: DontCare,
        }        
    }

    pub fn into_enabled_output(self) -> GpioConfig<Enabled, Output, PIN> {
        // call your hardware api to set the pin as output

        GpioConfig {
            pin: self.pin,
            enabled: Enabled,
            direction: Output,
        }
    }

    pub fn into_enabled_input(self) -> GpioConfig<Enabled, Input, PIN> {
        // call your hardware api to set the pin as input

        GpioConfig {
            pin: self.pin,
            enabled: Enabled,
            direction: Input,
        }
    }

    pub fn disable(self) -> GpioConfig<Disabled, DontCare, PIN> {
        // call your hardware api to disable the pin

        GpioConfig {
            pin: self.pin,
            enabled: Disabled,
            direction: DontCare,
        }
    }
}

// only enabled for input pins
impl<PIN: std::fmt::Debug> GpioConfig<Enabled, Input, PIN> {
    pub fn read(&self) -> bool {
        // call your hardware api to read the pin
        println!("Reading pin {:?}.", self.pin);
        true
    }
}

// only enabled for output pins
impl<PIN: std::fmt::Debug> GpioConfig<Enabled, Output, PIN> {
    pub fn set_high(&self) {
        // call your hardware api to set the pin to high
        println!("Setting pin {:?} to high.", self.pin)
    }

    pub fn set_low(&self) {
        // call your hardware api to set the pin to low
        println!("Setting pin {:?} to low.", self.pin)
    }
}

// problems?
//    - port and gpio can be swapped when calling new --> SOLVED
//    - we can have two structs setting up the same pin
//    - each GPIO takes 8 bytes ---> SOLVED

fn main() {
    let pin = GpioConfig::<Disabled, DontCare, _>::new(PORTA_0);
    let input_pin = pin.into_enabled_input();
    println!("read pin: {}", input_pin.read());

    let pin = GpioConfig::<Disabled, DontCare, _>::new(PORTB_0);
    let output_pin = pin.into_enabled_output();
    output_pin.set_high();

    println!("sizeof input pin: {:?}", std::mem::size_of_val(&input_pin));
    println!("sizeof output pin: {:?}", std::mem::size_of_val(&output_pin));
}
