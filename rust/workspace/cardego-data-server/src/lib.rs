use std::{hash::Hash, sync::Arc};

mod collections;
mod models;

pub type AppResult<T> = eyre::Result<T>;

pub trait Idable {
    type Id: Hash + Eq + Clone;

    fn uid(&self) -> &Self::Id;
}

impl<I, T> Idable for Arc<T>
where
    I: Hash + Eq + Clone,
    T: Idable<Id = I>,
{
    type Id = I;

    fn uid(&self) -> &Self::Id {
        self.as_ref().uid()
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
