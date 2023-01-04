use std::time::Duration;

use measure_time::measure_time;

fn main() {
    assert_eq!(sleep_seconds(1), 1);
}

#[measure_time]
fn sleep_seconds(seconds: u64) -> u64 {
    std::thread::sleep(Duration::from_secs(seconds));

    seconds
}