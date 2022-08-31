use crate::city::*;
use crate::resources::*;

use common_macros::hash_map;
use std::collections::HashMap;

pub type Coord = (u64, u64);

#[derive(Debug)]
pub enum Biome {
    Grass,
    Snow,
    Desert,
}

#[derive(Debug)]
pub enum Terrain {
    Flat,
    Hill,
    Mountain,
}

pub type TileCount = u64;

#[derive(Debug)]
pub struct Tile {
    biome: Biome,
    terrain: Terrain,
    resource: Option<(Resource, ResourceCount)>,
}

impl Tile {
    pub fn new(
        biome: Biome,
        terrain: Terrain,
        resource: Option<(Resource, ResourceCount)>,
    ) -> Self {
        Tile {
            biome,
            terrain,
            resource,
        }
    }

    pub fn tile_char(&self) -> char {
        match &self.resource {
            None => ' ',
            Some((Resource::Wood, _)) => 'w',
            Some((Resource::Bronze, _)) => 'b',
            Some((Resource::Stone, _)) => 's',
            Some((Resource::Coal, _)) => 'c',
            Some((Resource::Iron, _)) => 'i',
            Some((Resource::Steel, _)) => 't',
            Some((Resource::Uranium, _)) => 'u',
        }
    }

    pub fn char_mod(&self) -> (char, char) {
        match &self.terrain {
            Terrain::Mountain => ('^', '^'),
            Terrain::Hill => ('/', '\\'),
            Terrain::Flat => ('_', '_'),
        }
    }

    pub fn biome(&self) -> &Biome {
        return &self.biome;
    }

    pub fn terrain(&self) -> &Terrain {
        return &self.terrain;
    }

    pub fn resource(&self) -> &Option<(Resource, ResourceCount)> {
        return &self.resource;
    }
}

#[derive(Debug)]
pub struct Territory {
    tiles: HashMap<Coord, Tile>, // Map coordinates to tiles
}

impl Territory {
    pub fn new() -> Self {
        let tiles: HashMap<Coord, Tile> = HashMap::new();
        return Territory { tiles };
    }

    pub fn show_map(&self) -> () {
        println!("Map:");

        let mut out = String::from("");
        for (coord, tile) in self.tiles.iter() {
            let (lmod, rmod) = tile.char_mod();
            let tile_str = &format!(
                "{}{}{} {:?}:\t{:?}\t{:?}\t{:?}\n",
                lmod,
                tile.tile_char(),
                rmod,
                coord,
                tile.biome(),
                tile.terrain(),
                tile.resource()
            );
            out.insert_str(0, tile_str);
        }

        println!("{}", out);
    }

    pub fn add_tile(&mut self, k: Coord, v: Tile) -> () {
        self.tiles.insert(k, v);
    }

    pub fn resources(&self) -> ResourceList {
	let out = ResourceList::new();
	return out
    }
}
