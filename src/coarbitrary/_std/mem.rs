use coarbitrary::*;

use std::mem::{Discriminant, ManuallyDrop};

delegate_hash!([T] Discriminant<T>);
delegate_deref!([T: CoArbitrary] ManuallyDrop<T>);