pub fn parse_onoff(raw: String) -> bool {
    match raw.as_str() {
        "on" => { true }
        "off" => { false }
        _ => { panic!("expected on/off bool") }
    }
}

pub fn parse_yesno(raw: String) -> bool {
    match raw.as_str() {
        "yes" => { true }
        "no" => { false }
        _ => { panic!("expected yes/no bool") }
    }
}
