//! Implement Fallible Btree, As there is no try_reserve methods on btree, I add no choice but to fork the std implementation and change return types.
pub mod map;
pub use map::BTreeMap;

pub mod set;
pub use set::BTreeSet;

mod node;
mod search;
use alloc::collections::TryReserveError;

#[doc(hidden)]
trait Recover<Q: ?Sized> {
    type Key;

    fn get(&self, key: &Q) -> Option<&Self::Key>;
    fn take(&mut self, key: &Q) -> Option<Self::Key>;
    fn replace(&mut self, key: Self::Key) -> Result<Option<Self::Key>, TryReserveError>;
}
