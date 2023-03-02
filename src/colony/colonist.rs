use bevy::window::WindowPlugin;
use crate::colony::any;
use any::ColonyEntity;

struct Colonist {
    pub name: String
}

impl Colonist {

    pub fn new(name: String) -> Colonist {
        return Colonist {
            name
        }
    }
}

impl ColonyEntity for Colonist {

}