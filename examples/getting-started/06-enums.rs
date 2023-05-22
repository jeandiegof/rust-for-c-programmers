// An extremelly simple (but non efficient) protocol to exchange
// data between a sensor and an application. Data packets have the
// following format:
//
//    packet[0] => single byte containing the id of the packet
//    packet[1..] => data bytes (temperature, humidity or pressure)
//
use std::convert::TryInto;

#[derive(Debug)]
enum SensorData {
    Temperature(u8),
    Humidity(u8),
    Pressure(f32),
}

impl SensorData {
    const TEMPERATURE_ID: u8 = 0xB1;
    const HUMIDITY_ID: u8 = 0xB2;
    const PRESSURE_ID: u8 = 0xB3;

    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            SensorData::Temperature(t) | SensorData::Humidity(t) => vec![self.id(), t as u8],
            SensorData::Pressure(p) => {
                let mut encoded = vec![self.id()];
                encoded.extend_from_slice(&p.to_be_bytes());

                encoded
            }
        }
    }

    pub fn from_bytes(data: Vec<u8>) -> Self {
        match data[0] {
            Self::TEMPERATURE_ID => SensorData::Temperature(u8::from_be(data[1])),
            Self::HUMIDITY_ID => SensorData::Humidity(u8::from_be(data[1])),
            Self::PRESSURE_ID => SensorData::Pressure(f32::from_be_bytes(data[1..].try_into().unwrap())),
            _ => panic!("Packed ID is invalid.")
        }
    }

    fn id(&self) -> u8 {
        match self {
            SensorData::Temperature(_) => Self::TEMPERATURE_ID,
            SensorData::Humidity(_) => Self::HUMIDITY_ID,
            SensorData::Pressure(_) => Self::PRESSURE_ID,
        }
    }
}

fn main() {
    // the sending end
    let sensor_data = SensorData::Temperature(25);
    println!("sending {:?}", sensor_data.to_bytes());

    // the receiving end
    let raw_data = vec![0xB1, 0x19];
    let sensor_data = SensorData::from_bytes(raw_data);

    match sensor_data {
        SensorData::Temperature(t) => println!("received temp: {:?}", t),
        SensorData::Humidity(h) => println!("received humidity: {:?}", h),
        SensorData::Pressure(p) => println!("received pressure: {:?}", p)
    }
}