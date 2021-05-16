use std::time::{SystemTimeError, SystemTime, Duration, Instant};

pub fn timestamp_now() -> Result<u64, SystemTimeError> {
    return match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => { Ok(n.as_secs()) }
        Err(e) => { Err(e) }
    }
}

pub fn from_u64(timestamp: u64) -> SystemTime {
    return SystemTime::UNIX_EPOCH + Duration::from_secs(timestamp);
}

pub fn measure_begin() -> Instant {
    return Instant::now();
}

pub fn measure_end(measurement_instant: &Instant) -> Duration {
    return measurement_instant.elapsed();
}