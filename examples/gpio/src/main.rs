#![allow(dead_code)]

struct GpioConfig<ENABLED, DIRECTION> {
    port: u32,    
    gpio: u32,    
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

// functions available for all pins
impl<ENABLED, DIRECTION> GpioConfig<ENABLED, DIRECTION> {
    pub fn new(port: u32, gpio: u32) -> GpioConfig<Disabled, DontCare> {
        // call you hardware api here to mark the pin as disabled/high-z/whatever

        GpioConfig {
            port,
            gpio,
            enabled: Disabled,
            direction: DontCare,
        }        
    }

    pub fn into_enabled_output(self) -> GpioConfig<Enabled, Output> {
        // call your hardware api to set the pin as output

        GpioConfig {
            port: self.port,
            gpio: self.gpio,
            enabled: Enabled,
            direction: Output,
        }
    }

    pub fn into_enabled_input(self) -> GpioConfig<Enabled, Input> {
        // call your hardware api to set the pin as input

        GpioConfig {
            port: self.port,
            gpio: self.gpio,
            enabled: Enabled,
            direction: Input,
        }
    }

    pub fn disable(self) -> GpioConfig<Disabled, DontCare> {
        // call your hardware api to disable the pin

        GpioConfig {
            port: self.port,
            gpio: self.gpio,
            enabled: Disabled,
            direction: DontCare,
        }
    }
}

// only enabled for input pins
impl GpioConfig<Enabled, Input> {
    pub fn read(&self) -> bool {
        // call your hardware api to read the pin
        true
    }
}

// only enabled for output pins
impl GpioConfig<Enabled, Output> {
    pub fn set_high(&self) {
        // call your hardware api to set the pin to high
        println!("Setting pin {:x}:{} to high.", self.port, self.gpio)
    }

    pub fn set_low(&self) {
        // call your hardware api to set the pin to low
        println!("Setting pin {:x}:{} to high.", self.port, self.gpio)
    }
}

// problems?
//    - port and gpio can be swapped when calling new
//    - we can have two structs setting up the same pin
//    - each GPIO takes 8 bytes

fn main() {
    let pin = GpioConfig::<Disabled, DontCare>::new(0xFEFABBBB, 0);
    let input_pin = pin.into_enabled_input();
    println!("read pin: {}", input_pin.read());

    let pin = GpioConfig::<Disabled, DontCare>::new(0xFEFABBBB, 1);
    let output_pin = pin.into_enabled_output();
    output_pin.set_high();

    println!("sizeof disabled pin: {:?}", std::mem::size_of::<GpioConfig::<Disabled, DontCare>>());
    println!("sizeof disabled pin: {:?}", std::mem::size_of::<GpioConfig::<Enabled, Output>>());
    println!("sizeof disabled pin: {:?}", std::mem::size_of::<GpioConfig::<Enabled, Input>>());
}
