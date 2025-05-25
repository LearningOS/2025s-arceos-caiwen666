pub use alloc::collections::TryReserveError;
pub use alloc::collections::{BTreeMap, BTreeSet, BinaryHeap};
pub use alloc::collections::{LinkedList, VecDeque};
pub use alloc::collections::{binary_heap, btree_map, btree_set};
pub use alloc::collections::{linked_list, vec_deque};
#[doc(inline)]
pub use self::hash_map::HashMap;
pub use crate::ops::Bound;

mod hash;

pub mod hash_map {
    //! A hash map implemented with quadratic probing and SIMD lookup.
    pub use super::hash::map::*;
    pub use crate::hash::random::DefaultHasher;
    pub use crate::hash::random::RandomState;
}