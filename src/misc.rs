use colored::Colorize;

pub fn parse_onoff(raw: &str) -> bool {
    match raw {
        "on" => { true }
        "off" => { false }
        _ => { panic!("expected on/off bool") }
    }
}

pub fn parse_yesno(raw: &str) -> bool {
    match raw {
        "yes" => { true }
        "no" => { false }
        _ => { panic!("expected yes/no bool") }
    }
}

pub fn colorized_storage_si_suffix(value: u64) -> String {
    let mut count: u8 = 0;
    let mut state: f64 = value as f64;
    while state >= 1024. && count < 5 {
        state /= 1024.;
        count += 1;
    }
    let number = format!("{:.02}", state);
    if count == 0 {
        return format!("{} B", number).bright_cyan().to_string();
    }
    if count == 1 {
        return format!("{} KiB", number).bright_green().to_string();
    }
    if count == 2 {
        return format!("{} MiB", number).bright_yellow().to_string();
    }
    if count == 3 {
        return format!("{} GiB", number).bright_red().to_string();
    }
    if count == 4 {
        return format!("{} TiB", number).bright_magenta().to_string();
    }
    format!("{} PiB", number).bright_white().to_string()
}
