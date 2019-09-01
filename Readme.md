# w1\_therm\_reader: simple parser to read temp out of 1wire temperzture sensor

## Purpose

This lib aims at providing a simple way to read temperature out of a `w1_therm` compatible sensor connected to a linux machine. See the [kernel documentation](https://www.kernel.org/doc/Documentation/w1/slaves/w1_therm ) to know which devices are compatibles.

If you want a complete OneWire library, this crate is not the one you're looking for. Maybe [Onewire](https://crates.io/crates/onewire).

It was a way to give [nom](https://crates.io/crates/nom) a try.

## Status

This lib as passed the "work for me stage" but it has not been thoroughly tested. This is developed on my free tim and I'm not a seasoned developer. Feel free to create PRs.

## Usage

```
extern crate w1_therm_reader;
use w1_therm_reader::{read_from_file, read_from_device, convert_to_metric};

fn main() {
    // get the temperature from a file
    let t = read_from_file("/path/to/w1/").unwrap();
    // get the temperature from device
    let t = read_from_file("10-000001").unwrap();
    // get the temperature in °C instead of m°C
    let t_metric = convert_to_metric(t);
}
```
