use std::time::*;

delegate_hash!(Duration);
#[cfg(feature = "unstable")]
delegate_hash!(SystemTime);
#[cfg(feature = "unstable")]
delegate_hash!(Instant);

coarbitrary!(SystemTimeError; self, var => var.nest(&self.duration()));