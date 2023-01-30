use indexmap::IndexSet;

use std::hash::Hash;

pub trait Idable {
    type Id: Hash + Eq;

    fn uid(&self) -> &Self::Id;
}

// Attributes can have component attributes
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Id(pub String);

impl From<&str> for Id {
    fn from(other: &str) -> Self {
        Self(other.to_string())
    }
}

impl From<String> for Id {
    fn from(other: String) -> Self {
        Self(other)
    }
}

#[derive(Debug)]
pub struct Attribute {
    pub id: Id,
    pub name: String,
}

impl Idable for Attribute {
    type Id = Id;

    fn uid(&self) -> &Self::Id {
        &self.id
    }
}

#[derive(Debug)]
pub struct Entity {
    pub id: Id,
    pub name: String,
    pub attribute_ids: IndexSet<Id>,
}

impl Idable for Entity {
    type Id = Id;

    fn uid(&self) -> &Self::Id {
        &self.id
    }
}

// Items and Actions and Skills can be Simple or Complex
// Simple types can have N entities. But you cannot have more than 1 type of attribute that belongs
// to an action set.

// So, like, a Simple item cannot be both Wet and Dry in the same Component.

// We are defining an EntityModel in Cardego ultimately.
// A sword has an handle and blade component usually.
// The handle can be made out of wood
// And the blade can be made out of steel.
// And they are bound by the Form factor.
// Doing stuff to the sword can cause the different components to react differently.

// For the purposes of Cardego, sometimes you want to model this complexity, and sometimes you don't.
// - You can choose to insert complexity into the model by applying splitting rules.

// Fireball (ID: N)
// [Fire, Ranged, Attack, Burst]
//
// (the actual guts of how a card works can be left out, this can be left for another day)
//
// but, in general, abilities should NOT be turing complete! Therefore, they should be complete
// programs.
//
// Abilities should be presented as coroutines with limited scope and power.
