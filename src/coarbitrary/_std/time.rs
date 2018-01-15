use std::time::*;

delegate_hash!(Duration);
delegate_hash!(Instant);
delegate_hash!(SystemTime);

coarbitrary!(SystemTimeError; self, var => var.nest(&self.duration()));