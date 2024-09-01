pub fn parse_onoff(raw: String) -> bool {
    match raw.as_str() {
        "on" => { true }
        "off" => { false }
        _ => { panic!("expected on/off bool") }
    }
}
