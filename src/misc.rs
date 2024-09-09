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
