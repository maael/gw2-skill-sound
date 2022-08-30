use crate::link::{GW2Identity, GW2};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkContext {
    pub map_id: u16,
    pub player_x: f32,
    pub player_y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkedMem {
    pub context: LinkContext,
    pub identity: GW2Identity,
}

pub fn get_link() -> GW2Identity {
    let mut gw2 = GW2::new().expect("Unable to link to Guild Wars 2");
    let link = gw2.tick().unwrap();
    serde_json::from_str(&link.identity().to_string()).unwrap()
}
