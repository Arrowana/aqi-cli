extern crate reqwest;
extern crate serde_json;
extern crate structopt;

mod models;
mod compute;
mod parsing;

use models::{Location, SensorData, SensorDataValue, FreeGeoIPLocation};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aqi cli", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short, long)]
    verbose: bool,

    #[structopt(short, long, help="Maximum distance to included sensor for calculation", default_value="5.0")]
    distance: f32,

    #[structopt(long, required_if("longitude", "latitude"))]
    latitude: Option<f32>,

    #[structopt(long, required_if("latitude", "longitude"))]
    longitude: Option<f32>,

    #[structopt(short, long)]
    input: Option<String>,
}

fn get_current_location() -> Result<Location, reqwest::Error> {
    let geolocation: FreeGeoIPLocation = reqwest::blocking::get("https://freegeoip.app/json/")?.json()?;

    Ok(Location { latitude: geolocation.latitude, longitude: geolocation.longitude })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    
    // Location { latitude: -33.915120, longitude: 151.204064 }
    let current_location = match (opt.latitude, opt.longitude) {
        (Some(lat), Some(long)) => Location { latitude: lat, longitude: long },
        _ => get_current_location()?,
    };
    println!("Location is: {},{}", current_location.latitude, current_location.longitude);

    let data: Vec<SensorData> = match &opt.input {
        Some(input) => {
            let file = std::fs::File::open(input)?;
            serde_json::from_reader(file)?
        },
        None => reqwest::blocking::get("https://maps.sensor.community/data/v2/data.24h.json")?.json()?
    };

    let selected_sensor_data_values: Vec<SensorDataValue> = data.iter().filter_map(|sensor_data| {
        let distance = compute::calculate_distance_to_location(&sensor_data.location, &current_location);
        if distance < opt.distance {
            Some(sensor_data.sensordatavalues.clone())
        } else {
            None
        }
    }).flatten().collect();

    if opt.verbose {
        println!("Found a total of {} sensor locations, {} within {} km", data.len(), selected_sensor_data_values.len(), opt.distance);
        (&selected_sensor_data_values).iter().for_each(|sensor_data_value| println!("{} {}", sensor_data_value.value, sensor_data_value.value_type));
    }

    println!(
        "P10: {}, P2.5: {}",
        compute::compute_average_data_value(&selected_sensor_data_values, "P1"),
        compute::compute_average_data_value(&selected_sensor_data_values, "P2"),
    );

    Ok(())
}