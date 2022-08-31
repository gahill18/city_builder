use crate::city::Satisfaction;

use common_macros::hash_map;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Resource {
    Wood,
    Coal,
    Stone,
    Bronze,
    Iron,
    Steel,
    Uranium,
}

pub type ResourceCount = u32;
pub type ResourceWeight = f32;

#[derive(Debug)]
pub struct ResourceList {
    resources: HashMap<Resource, ResourceCount>,
    resource_weights: HashMap<Resource, ResourceWeight>,
    resource_max: HashMap<Resource, ResourceCount>,
}

use crate::resources::Resource::*;
impl ResourceList {
    pub fn new() -> Self {
        let resources: HashMap<Resource, ResourceCount> = hash_map! {} ;
        let resource_weights: HashMap<Resource, Satisfaction> = hash_map! {
            Wood    => 0.0,
            Coal    => 0.25,
            Stone   => 0.2,
            Bronze  => 0.4,
            Iron    => 0.6,
            Steel   => 0.7,
            Uranium => 1.0,
        };
	let resource_max = hash_map! {
	    Wood    => (10 as ResourceCount).pow(4),
            Coal    => (10 as ResourceCount).pow(4),
            Stone   => (10 as ResourceCount).pow(4),
            Bronze  => (10 as ResourceCount).pow(4),
            Iron    => (10 as ResourceCount).pow(4),
            Steel   => (10 as ResourceCount).pow(4),
	    Uranium => (10 as ResourceCount).pow(4),
	};
        return ResourceList {
            resources,
            resource_weights,
	    resource_max
        };
    }

    pub fn show_resources(&self) -> () {
        println!("Resources:\n {:?}", self.resources);
    }

    pub fn satisfaction(&self) -> Satisfaction {
        let mut out: Satisfaction = 0.0;
        for (resource, resource_count) in self.resources.iter() {
            out += (*resource_count as f32 * self.resource_weights[resource]) as f32
        }
        return out;
    }
}
