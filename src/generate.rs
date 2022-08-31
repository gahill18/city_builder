use rand::distributions::Standard;
use rand::prelude::*;

use crate::city::*;
use crate::geography::*;
use crate::resources::*;
use crate::resources::Resource::*;
use crate::helpers::*;

pub fn generate_city(name: String, popcap: Population, starting_tiles: TileCount) -> City {
    let mut rng = rand::thread_rng();

    // Randomized parts
    let pop = generate_population(&mut rng, popcap);
    let ter = generate_territory(&mut rng, starting_tiles);

    let new_city: City = City::new(name, pop, ter);
    return new_city;
}

pub fn generate_territory(rng: &mut ThreadRng, starting_tiles: TileCount) -> Territory {
    let mut territory = Territory::new();
    let mut coords: Vec<Coord> = generate_coords(starting_tiles);
    let mut tiles: Vec<Tile> = generate_tiles(rng, starting_tiles);

    while !coords.is_empty() && !tiles.is_empty() {
        match (coords.pop(), tiles.pop()) {
            (Some(c), Some(t)) => territory.add_tile(c, t),
            (bad_c, bad_t) => println!("failed to add tile {:?} at coord {:?}", bad_c, bad_t),
        }
    }

    return territory;
}

// Currently a square, takes top left and bottom right coordinate and populates the list
pub fn generate_coords(starting_tiles: TileCount) -> Vec<Coord> {
    let mut coords = Vec::new();

    // Calculate boundaries
    let square_boundary_length: u64 = f64::sqrt(starting_tiles as f64) as u64 + 1;
    for x in 0..square_boundary_length {
        for y in 0..square_boundary_length {
            coords.push((x, y));
        }
    }

    return coords;
}

pub fn generate_tiles(rng: &mut ThreadRng, num: TileCount) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = Vec::new();
    for _i in 0..num {
        let (biome, terrain, resource) = (
            generate_biome(rng),
            generate_terrain(rng),
            generate_resource(rng),
        );
        let tile = Tile::new(biome, terrain, resource);
        tiles.push(tile);
    }

    return tiles;
}

impl Distribution<Biome> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Biome {
        match rng.gen_range(0, 2 + 1) {
            0 => Biome::Grass,
            1 => Biome::Snow,
            2 => Biome::Desert,
            _ => todo!(),
        }
    }
}

fn generate_biome(rng: &mut ThreadRng) -> Biome {
    return rng.gen::<Biome>();
}

impl Distribution<Terrain> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Terrain {
        match rng.gen_range(0, 3) {
            0 => Terrain::Flat,
            1 => Terrain::Hill,
            2 => Terrain::Mountain,
            _ => todo!(),
        }
    }
}

fn generate_terrain(rng: &mut ThreadRng) -> Terrain {
    return rng.gen::<Terrain>();
}

impl Distribution<Resource> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Resource {
        match rng.gen_range(0, 7) {
            0 => Wood,
            1 => Coal,
            2 => Stone,
            3 => Bronze,
            4 => Iron,
            5 => Steel,
            6 => Uranium,
            _ => todo!(),
        }
    }
}

const RESOURCE_MAX: ResourceCount = (10 as ResourceCount).pow(4);
fn generate_resource(rng: &mut ThreadRng) -> Option<(Resource, ResourceCount)> {
    match rng.gen_range(0, 3) {
        0 => {
	    let r: Resource = rng.gen();
	    let count: ResourceCount = rng.gen_range(1, RESOURCE_MAX);
	    Some((r, count))
	}
        _ => None,
    }
}

fn generate_population(rng: &mut ThreadRng, popcap: Population) -> Population {
    return rng.gen_range(0, popcap + 1);
}
