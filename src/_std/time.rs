use super::*;
use std::time::*;

impl_arbitrary!(Duration, SMapped<'a, (u64, u32), Self>,
    static_map(any::<(u64, u32)>(), |(a, b)| Duration::new(a, b))
);

// Instant::now() "never" returns the same Instant, so no shrinking may occur!
impl_just!(Instant, Instant::now());
// Same for SystemTime.
impl_just!(SystemTime, SystemTime::now());
/*
A possible logic for SystemTimeError:
fn gen_ste() -> SystemTimeError {
    (SystemTime::now() + Duration::from_millis(10)).elapsed().unwrap_err()
}
This may however panic from time to time. NTP could also ruin our day!
*/