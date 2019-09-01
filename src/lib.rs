//! # `w1_therm_reader`
//! A lib to parse w1_therm /sys file on linux to get data from onewire temperature sensors
//! exemples are in the Readme.md file
extern crate nom;
pub mod parser;

use std::fs;
use std::io::{Error, ErrorKind, Result};

/// A struct with datas from sensor
/// * crc is whether crc is correct
/// * temp is the temp in m°C
#[derive(Debug)]
pub struct ResultW1 {
    pub crc: bool,
    pub temp: i32,
    raw_value: String,
}

fn validate_w1(result: ResultW1) -> Result<i32> {
    if result.crc {
        Ok(result.temp)
    } else {
        Err(Error::new(ErrorKind::InvalidData, "CRC Error"))
    }
}

fn handle_nom_error() -> Result<i32> {
    Err(Error::new(ErrorKind::InvalidData, "Parse Error"))
}

/// An helper to convert data from m°C to °C
/// # Example
/// ```
/// assert_eq!(w1_therm_reader::convert_to_metric(-22344),-22.344)
/// ```

pub fn convert_to_metric(read_temp: i32) -> f32 {
    (read_temp as f32) / 1000.00
}

/// A shortcut to read from device id as a string
/// Returns an io::Result<i32> with the temperature in m°C or an Error.
/// Calls `read_from_file` internally
/// # Example
/// ```
/// let temp = w1_therm_reader::read_from_probe("10-ae1234fff");
/// ```
pub fn read_from_probe(id: &str) -> Result<i32> {
    let path = format!("/sys/bus/w1/devices/{}/w1_slave", id);
    read_from_file(&path)
}

/// Read from a file the temperature
/// It takes a path as a String and returns an io::Result<i32> with the temperature in m°C or an Error.
/// # Example
/// ```
/// let temp = w1_therm_reader::read_from_file("assets/w1_ex.txt").unwrap();
/// assert_eq!(temp, -22187);
/// ```
pub fn read_from_file(filename: &str) -> Result<i32> {
    let content = fs::read_to_string(filename)?;
    match parser::parser(&content) {
        Ok((_, res)) => validate_w1(res),
        Err(_x) => handle_nom_error(),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_convert() {
        assert_eq!(convert_to_metric(22867), 22.867);
    }
    #[test]
    fn test_convert2() {
        assert_eq!(convert_to_metric(-22847), -22.847);
    }
    #[test]
    fn test_read_ok() {
        assert_eq!(read_from_file("assets/w1_ex.txt").unwrap(), -22187);
    }
    #[test]
    fn test_read_crc_error() {
        assert!(read_from_file("assets/w2_ex.txt").is_err());
    }
    #[test]
    fn test_read_nonexistent_error() {
        assert!(read_from_file("assets/not_here.txt").is_err());
    }

}
