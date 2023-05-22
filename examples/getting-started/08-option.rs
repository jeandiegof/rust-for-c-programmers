struct SensorData {
    value: u64,
}

// Imagine a function that reads data from a sensor connected to the
// processor running this code. What if there is no data available?
fn read_sensor_data() -> SensorData {
    unimplemented!()
}

fn main() {
    let sensor_data =  read_sensor_data();

    println!("{:?}", sensor_data.value);
}
