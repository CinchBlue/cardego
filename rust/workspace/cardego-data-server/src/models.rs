use std::hash::Hash;

pub mod types;

pub trait Idable {
    type Id: Hash + Eq;

    fn uid(&self) -> Self::Id;
}
