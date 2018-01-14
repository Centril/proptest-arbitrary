use coarbitrary::*;

use std::iter::*;

delegate_iter!([ T: CoArbitrary
               , A: Clone + Iterator<Item = T>
               , B: Clone + Iterator<Item = T>]
               Chain<A, B>);

delegate_iter!(['a
               , T: 'a + Clone + CoArbitrary
               , A: Clone + Iterator<Item = &'a T>]
               Cloned<A>);

impl<A> CoArbitrary for Empty<A> {
    fn coarbitrary(&self, _: Perturbable) {}
}

delegate_iter!([ T: CoArbitrary, A: Clone + Iterator<Item = T>] Enumerate<A>);

delegate_iter!([ T: CoArbitrary, A: Clone + Iterator<Item = T>] Fuse<A>);

delegate_iter!([ A: CoArbitrary
               , P: Clone + FnMut(&A) -> bool
               , I: Clone + Iterator<Item = A>]
               Filter<I, P>);

delegate_iter!([ B: CoArbitrary
               , F: Clone + FnMut(<I as Iterator>::Item) -> Option<B>
               , I: Clone + Iterator]
               FilterMap<I, F>);

delegate_iter!([ I: Clone + Iterator
               , T: Clone + CoArbitrary
               , J: Clone + Iterator<Item = T>
               , U: Clone + IntoIterator<Item = T, IntoIter = J>
               , F: Clone + FnMut(<I as Iterator>::Item) -> U]
               FlatMap<I, U, F>);

delegate_iter!([ T: CoArbitrary
               , F: Clone + FnMut(&T) -> ()
               , I: Clone + Iterator<Item = T>]
               Inspect<I, F>);

delegate_iter!([ B: CoArbitrary
               , F: Clone + FnMut(<I as Iterator>::Item) -> B
               , I: Clone + Iterator]
               Map<I, F>);

delegate_iter!([ A: Clone + CoArbitrary] Once<A>);

delegate_iter!([ T: Clone + CoArbitrary
               , A: Clone + Iterator<Item = T>]
               Peekable<A>);

delegate_iter!([ T: CoArbitrary
               , A: Clone + DoubleEndedIterator<Item = T>]
               Rev<A>);

delegate_iter!([ B: CoArbitrary
               , I: Clone + Iterator
               , St: Clone
               , F: Clone + FnMut(&mut St, <I as Iterator>::Item) -> Option<B>]
               Scan<I, St, F>);

delegate_iter!([ T: CoArbitrary, A: Clone + Iterator<Item = T>] Skip<A>);

delegate_iter!([ T: CoArbitrary
               , I: Clone + Iterator<Item = T>
               , P: Clone + FnMut(&T) -> bool]
               SkipWhile<I, P>);

delegate_iter!([ T: CoArbitrary, A: Clone + Iterator<Item = T>] Take<A>);

delegate_iter!([ T: CoArbitrary
               , I: Clone + Iterator<Item = T>
               , P: Clone + FnMut(&T) -> bool]
               TakeWhile<I, P>);

delegate_iter!([ T: CoArbitrary
               , A: Clone + Iterator<Item = T>
               , B: Clone + Iterator<Item = T>]
               Zip<A, B>);

#[cfg(feature = "unstable")]
delegate_iter!([ T: CoArbitrary, A: Clone + Iterator<Item = T>] StepBy<A>);