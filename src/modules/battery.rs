use ansi_term::Color;

use super::{Context, Module};

/// Creates a module for the battery percentage and charging state
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const BATTERY_FULL: &str = "•";
    const BATTERY_CHARGING: &str = "⇡";
    const BATTERY_DISCHARGING: &str = "⇣";
    const BATTERY_UNKNOWN: &str = "?";
    const BATTERY_THRESHOLD: f64 = 100.0;

    let battery_status = get_battery_status()?;
    let BatteryStatus { state, percentage, cycle } = battery_status;

    let mut module = context.new_module("battery")?;
    let battery_threshold = module.config_value_f64("threshold").unwrap_or(BATTERY_THRESHOLD) as f32;
    let show_cycle = module.config_value_bool("show_cycle").unwrap_or(false);

    if percentage > battery_threshold {
        log::debug!(
            "Battery percentage is higher than threshold ({} > {})",
            percentage,
            battery_threshold
        );
        return None;
    }

    // TODO: Set style based on percentage when threshold is modifiable
    module.set_style(Color::Red.bold());
    module.get_prefix().set_value("");

    match state {
        battery::State::Full => {
            module.new_segment("full_symbol", BATTERY_FULL);
        }
        battery::State::Charging => {
            module.new_segment("charging_symbol", BATTERY_CHARGING);
        }
        battery::State::Discharging => {
            module.new_segment("discharging_symbol", BATTERY_DISCHARGING);
        }
        battery::State::Unknown => {
            module.new_segment("unknown_symbol", BATTERY_UNKNOWN);
        }
        _ => return None,
    }

    let mut percent_string = Vec::<String>::with_capacity(5);
    // Round the percentage to a whole number
    percent_string.push(percentage.round().to_string());
    percent_string.push("%%".to_string());
    if show_cycle {
        percent_string.push(" ♻️  ".to_string());
        percent_string.push(cycle.to_string());
        percent_string.push("cycles".to_string());
    }
    module.new_segment("percentage", percent_string.join("").as_ref());

    Some(module)
}

fn get_battery_status() -> Option<BatteryStatus> {
    let battery_manager = battery::Manager::new().ok()?;
    match battery_manager.batteries().ok()?.next() {
        Some(Ok(battery)) => {
            log::debug!("Battery found: {:?}", battery);
            let battery_status = BatteryStatus {
                percentage: battery.state_of_charge().value * 100.0,
                cycle: battery.cycle_count().unwrap(),
                state: battery.state(),
            };

            Some(battery_status)
        }
        Some(Err(e)) => {
            log::debug!("Unable to access battery information:\n{}", &e);
            None
        }
        None => {
            log::debug!("No batteries found");
            None
        }
    }
}

struct BatteryStatus {
    percentage: f32,
    cycle: u32,
    state: battery::State,
}
