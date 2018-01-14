use coarbitrary::*;

use std::str::*;

impl CoArbitrary for ParseBoolError {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&());
    }
}

impl CoArbitrary for Utf8Error {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.valid_up_to()).nest(&self.error_len());
    }
}

delegate_iter!(['a] Bytes<'a>);
delegate_iter!(['a] CharIndices<'a>);
delegate_iter!(['a] Chars<'a>);
delegate_iter!(['a] EncodeUtf16<'a>);
delegate_iter!(['a] Lines<'a>);

// std::str::LinesAny is deprecated.

delegate_iter!(['a] SplitWhitespace<'a>);

#[cfg(feature = "unstable")]
use std::str::pattern::*;

#[cfg(feature = "unstable")]
impl CoArbitrary for SearchStep {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            SearchStep::Match(ref x, ref y) => var.variant(0).nest(x).nest(y),
            SearchStep::Reject(ref x, ref y) => var.variant(1).nest(x).nest(y),
            SearchStep::Done => var.variant(2),
        };
    }
}

#[cfg(feature = "unstable")]
delegate_iter!(['a, S: Clone + ReverseSearcher<'a>, P: Pattern<'a, Searcher = S>]
               RMatchIndices<'a, P>);

#[cfg(feature = "unstable")]
delegate_iter!(['a, S: Clone + ReverseSearcher<'a>, P: Pattern<'a, Searcher = S>]
               RMatches<'a, P>);

#[cfg(feature = "unstable")]
delegate_iter!(['a, S: Clone + ReverseSearcher<'a>, P: Pattern<'a, Searcher = S>]
               RSplit<'a, P>);

#[cfg(feature = "unstable")]
delegate_iter!(['a, S: Clone + ReverseSearcher<'a>, P: Pattern<'a, Searcher = S>]
               RSplitN<'a, P>);

#[cfg(feature = "unstable")]
delegate_iter!(['a, S: Clone + ReverseSearcher<'a>, P: Pattern<'a, Searcher = S>]
               RSplitTerminator<'a, P>);

#[cfg(feature = "unstable")]
delegate_iter!(['a, S: Clone + Searcher<'a>, P: Pattern<'a, Searcher = S>]
               MatchIndices<'a, P>);

#[cfg(feature = "unstable")]
delegate_iter!(['a, S: Clone + Searcher<'a>, P: Pattern<'a, Searcher = S>]
               Matches<'a, P>);

#[cfg(feature = "unstable")]
delegate_iter!(['a, S: Clone + Searcher<'a>, P: Pattern<'a, Searcher = S>]
               Split<'a, P>);

#[cfg(feature = "unstable")]
delegate_iter!(['a, S: Clone + Searcher<'a>, P: Pattern<'a, Searcher = S>]
               SplitN<'a, P>);

#[cfg(feature = "unstable")]
delegate_iter!(['a, S: Clone + Searcher<'a>, P: Pattern<'a, Searcher = S>]
               SplitTerminator<'a, P>);