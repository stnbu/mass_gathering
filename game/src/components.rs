/// Components common to all packages (sort of)
use crate::*;
use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

// FIXME: Should there be a "Nickname" type? I think there probably should.
// Do that and also stop using `String` directly.
// NOTE: Such a type could be isomorphic to "client ID".

// FIXME: HEY HEY. Let's call it "player". Uses of "user" will be replaced by "player".
// Some uses of "nickname" can be replaced by "player". Interesting generative content
// could be accessible by index: `get_colors()` etc.

#[derive(
    Default, Serialize, Deserialize, Resource, Debug, Copy, Clone, Component, Eq, PartialEq,
)]
pub struct Player {
    id: u64,
}

impl Player {
    pub fn from_name(name: &str) -> Self {
        // FIXME: Return `Result`, Figure out immutable body.
        let mut name_vec = [b' '; 8];
        if name.len() > 8 {
            panic!()
        }
        for (i, c) in name.as_bytes().iter().enumerate() {
            name_vec[i] = *c;
        }
        let id = u64::from_ne_bytes(name_vec);
        Self { id }
    }

    pub fn from_id(id: u64) -> Self {
        Self { id }
    }

    pub fn get_name(&self) -> String {
        let name_vec: Vec<u8> = self.id.to_ne_bytes().to_vec();
        String::from_utf8(name_vec).unwrap().trim_end().to_string()
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

impl From<Player> for u64 {
    fn from(player: Player) -> Self {
        player.id
    }
}

#[derive(Component)]
pub struct MassID(pub u64);

#[derive(Serialize, Deserialize, Resource, Debug, Copy, Clone, Component)]
pub enum Inhabitation {
    Uninhabitable,
    Inhabitable(Option<Player>),
}

impl Inhabitation {
    pub fn by(&self, player: Player) -> bool {
        match self {
            &Self::Inhabitable(Some(value)) if value == player => true,
            _ => false,
        }
    }

    pub fn vacant(&self) -> bool {
        match self {
            Self::Inhabitable(None) => true,
            _ => false,
        }
    }

    pub fn uninhabitable(&self) -> bool {
        match self {
            Self::Uninhabitable => true,
            _ => false,
        }
    }

    pub fn inhabitable(&self) -> bool {
        !self.uninhabitable()
    }
}

#[derive(Component, Debug, Default)]
pub struct Momentum {
    pub velocity: Vec3,
}

// FIXME: Very client-specific. Relocate? Justification? Paperwork? Bike shaving?
/// Sights as in "gun sights"
#[derive(Component)]
pub struct Sights;
