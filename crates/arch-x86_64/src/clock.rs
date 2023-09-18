use core::time::Duration;

use hyperion_drivers::acpi::hpet::HPET;

//

pub fn main_counter_tick() -> u64 {
    HPET.main_counter_value()
}

pub fn from_ticks(tick: u64) -> Duration {
    Duration::from_nanos(HPET.ticks_to_nanos_u(tick))
}

pub fn to_ticks(duration: Duration) -> u64 {
    HPET.nanos_to_ticks_u(duration.subsec_nanos() as u64)
}

/* pub fn sleep() {
    let tim = HPET.next_timer();
    tim.sleep_until(deadline);
} */
