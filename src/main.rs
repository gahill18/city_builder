use crate::city::*;
use crate::generate::*;

pub mod city;
pub mod geography;
pub mod resources;
pub mod generate;
pub mod helpers;


const POPULATION_CAP: u64 = 10_u64.pow(9);
const STARTING_TILES: u64 = 10;

fn main() {
    // Non randomized parts
    let mut new_city: City = generate_city(String::from("New Donk City"), POPULATION_CAP, STARTING_TILES);
    new_city.greet();

    let mut escape = false;
    while !escape {
	match new_city.step() {
	    Ok(()) => {
		escape = true;
	    },
	    Err(s) => {
		println!("failed to step: {s}");
		escape = true;
	    },
	}
    }
    
}
