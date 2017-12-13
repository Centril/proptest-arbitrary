use super::*;
use std::time::*;

arbitrary!(Duration, SMapped<'a, (u64, u32), Self>;
    static_map(any::<(u64, u32)>(), |(a, b)| Duration::new(a, b))
);

// Instant::now() "never" returns the same Instant, so no shrinking may occur!
arbitrary!(Instant; Self::now());

// Same for SystemTime.
arbitrary!(SystemTime; Self::now());

/*
A possible logic for SystemTimeError:
fn gen_ste() -> SystemTimeError {
    (SystemTime::now() + Duration::from_millis(10)).elapsed().unwrap_err()
}
This may however panic from time to time. NTP could also ruin our day!
*/