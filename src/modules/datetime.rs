use ansi_term::Color;
use std::process::Command;
use chrono::Local;

use super::{Context, Module};

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const TIME_SYMBOL: &str = "🕒 ";
    let module_color = Color::Blue.bold();
    let datetime = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();

    let mut module = context.new_module("time")?;
    module.set_style(module_color);
    module.new_segment("symbol", TIME_SYMBOL);
    module.new_segment("datetime", &datetime);
    module.get_prefix().set_value("at ");

    Some(module)
}
