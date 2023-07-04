fn main() {
    let converted_value = convert(40.5, 'C');

    println!("The converted value is {}", converted_value);
}

fn convert(temp: f64, units: char) -> f64 {
    if units == 'F' {
        return (temp - 32.0) * (5.0 / 9.0);
    } else {
        return temp * (9.0 / 5.0) + 32.0;
    }
}
