use super::models;

fn degrees_to_radians(angle: f32) -> f32 {
    angle * std::f32::consts::PI / 180.0
}

pub fn calculate_distance_to_location(location1: &models::Location, location2: &models::Location) -> f32 {
    // adapted from js https://stackoverflow.com/questions/365826/calculate-distance-between-2-gps-coordinates
    let earth_radius_km = 6371.0;

    let delta_phi = degrees_to_radians(location2.latitude - location1.latitude);
    let delta_lambda = degrees_to_radians(location2.longitude - location1.longitude);
  
    let phi1 = degrees_to_radians(location1.latitude);
    let phi2 = degrees_to_radians(location2.latitude);
    
    let a = (delta_phi / 2.0).sin() * (delta_phi / 2.0).sin() +
            (delta_lambda / 2.0).sin() * (delta_lambda / 2.0).sin() * phi1.cos() * phi2.cos();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    earth_radius_km * c
}

pub fn compute_average_data_value(sensor_data_values: &Vec<models::SensorDataValue>, accept_value_type: &str) -> f32 {
    let selected_sensor_data_values: Vec<f32> = sensor_data_values.iter().filter_map(|sensor_data_value| {
        if sensor_data_value.value_type == accept_value_type {
            Some(sensor_data_value.value)
        } else {
            None
        }
    }).collect();

    println!("{}", selected_sensor_data_values.len());
    
    average(&selected_sensor_data_values)
}

fn average(numbers: &Vec<f32>) -> f32 {
    numbers.iter().sum::<f32>() / numbers.len() as f32
}