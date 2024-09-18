use noether::{
    AssociativeAddition, AssociativeMultiplication, BooleanAlgebra, CommutativeAddition,
    CommutativeMultiplication, ComplementedLattice, DistributiveLattice, JoinSemilattice,
    MeetSemilattice, BoundedLattice
};
use std::ops::{BitAnd, BitOr, Not};

/// The trait hierarchy in Noether is useful for verifying the correct and idiomatic implementation
/// of algebraic structures in Rust.
///
/// We take here, as an example, the skeleton implementation of a Boolean algebra.
/// This implementation demonstrates how to create a simple Boolean type that satisfies
/// all the properties of a Boolean algebra, including:
/// - Complementation (NOT operation)
/// - Join and Meet operations (OR and AND)
/// - Distributivity
/// - Bounded lattice structure (with top and bottom elements)
///
/// This example showcases how Noether's traits can be used to build up complex
/// algebraic structures from more fundamental properties, ensuring that all
/// necessary axioms are satisfied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bool(bool);
//todo!! 25-41
impl Bool {}

impl BitAnd for Bool{}

impl BitOr for Bool{}

impl Not for Bool{}

impl JoinSemilattice for Bool {}

impl MeetSemilattice for Bool {}

impl BoundedLattice for Bool {}

impl ComplementedLattice for Bool {}

impl DistributiveLattice for Bool {}


impl BooleanAlgebra for Bool {
    fn not(&self) -> Self {
        !*self
    }

    fn xor(&self, other: &Self) -> Self {
        Bool(self.0 ^ other.0)
    }

    fn implies(&self, other: &Self) -> Self {
        !*self | *other
    }

    fn equiv(&self, other: &Self) -> Self {
        Bool(self.0 == other.0)
    }

    fn is_true(&self) -> bool {
        self.0
    }

    fn is_false(&self) -> bool {
        !self.0
    }
}