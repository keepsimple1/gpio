use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

fn main() {
    println!("Hello, world!");
    let red_led = Pin::new(12);
    red_led.with_exported(|| {
        red_led.set_direction(Direction::Out).unwrap();
        red_led.set_value(1).unwrap();
        sleep(Duration::from_millis(500));
        red_led.set_value(0).unwrap();
        Ok(())
    }).unwrap();
}
