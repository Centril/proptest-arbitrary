use coarbitrary::*;

use std::slice::*;

delegate_iter!(['a, T: CoArbitrary] Chunks<'a, T>);
delegate_iter!(['a, T: CoArbitrary] Iter<'a, T>);
delegate_iter!(['a, T: CoArbitrary] Windows<'a, T>);

#[cfg(feature = "unstable")]
delegate_iter!([ 'a
               , T: Clone + CoArbitrary
               , P: Clone + FnMut(&T) -> bool]
               RSplit<'a, T, P>);

delegate_iter!([ 'a
               , T: CoArbitrary
               , P: Clone + FnMut(&T) -> bool]
               Split<'a, T, P>);