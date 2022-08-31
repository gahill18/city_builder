use crate::geography::*;
use crate::resources::*;
use crate::helpers::*;

pub type Satisfaction = f32;
pub type Population = u64;
pub const DISSATISFACTION_WEIGHT: Satisfaction = 0.001;

#[derive(Debug)]
pub struct City {
    name: String,                      // City name
    population: Population,            // Total population
    excess_satisfaction: Satisfaction, // How satisfied the population is overall
    territory: Territory,              // The territory the city controls
    resources: ResourceList,           // The resources the city currently has access to
}

impl City {
    pub fn new(
        name: String,
        population: Population,
        territory: Territory,
    ) -> Self {
	let (excess_satisfaction, resources) = (0.0, ResourceList::new());
        return City {
            name,
            population,
	    excess_satisfaction,
            territory,
	    resources
        };
    }

    pub fn greet(&self) -> () {
        println!("Welcome to {}!", self.name);
        println!("Population: {}", pretty_int(self.population));
        println!("Overall Satisfaction: {}", self.excess_satisfaction());
        self.show_resources();
        self.show_map();
    }

    fn show_map(&self) -> () {
        self.territory.show_map()
    }

    fn show_resources(&self) -> () {
        self.resources.show_resources()
    }

    fn name(&self) -> &String {
        return &self.name
    }

    fn population(&self) -> Population {
        return self.population
    }

    fn satisfaction(&self) -> Satisfaction {
	return self.resources.satisfaction()
    }

    fn dissatisfaction(&self) -> Satisfaction {
	return self.population as f32 * DISSATISFACTION_WEIGHT
    }

    fn excess_satisfaction(&self) -> Satisfaction {
        return self.satisfaction() - self.dissatisfaction()
    }

    fn resources(&self) -> &ResourceList {
        return &self.resources
    }

    fn territory(&self) -> &Territory {
        return &self.territory
    }

    pub fn step(&mut self) -> Result<(), String> {
        self.excess_satisfaction = self.excess_satisfaction();
	self.resources = self.territory.resources();
        return Ok(());
    }
}
