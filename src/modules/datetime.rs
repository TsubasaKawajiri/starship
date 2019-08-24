use ansi_term::Color;
use std::process::Command;
use chrono::Local;

use super::{Context, Module};

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const TIME_SYMBOL: &str = "🕒 ";
    const DEFAULT_FORMAT: &str = "%Y/%m/%d %H:%M:%S";

    let mut module = context.new_module("datetime")?;

    let format = module.config_value_str("format").unwrap_or(DEFAULT_FORMAT);
    let datetime = Local::now().format(format).to_string();

    module.set_style(Color::Blue.bold());
    module.new_segment("symbol", TIME_SYMBOL);
    module.new_segment("datetime", &datetime);
    module.get_prefix().set_value("at ");

    Some(module)
}
