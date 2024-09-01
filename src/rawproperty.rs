
#[derive(Debug)]
pub struct RawProperty {
    pub dataset: String,
    pub name: String,
    pub value: String,
    pub meta: String,
}

impl RawProperty {

    pub fn from_line(line: &str) -> Self{

        let mut fragments = line.split("\t");

        let raw_property = Self {
            dataset: fragments.next().unwrap().to_string(),
            name: fragments.next().unwrap().to_string(),
            value: fragments.next().unwrap().to_string(),
            meta: fragments.next().unwrap().to_string(),
        };

        raw_property

    }

    pub fn from_raw(raw: &String) -> Vec<Self> {

        let lines = raw.split("\n");
        let chars: &[_] = &[' ', '\t'];
        let mut raw_properties: Vec<Self> = Vec::new();

        for line in lines {
            let line_cleaned = line.trim_matches(chars);
            if line_cleaned.len() == 0 {
                continue;
            }
            let raw_property = Self::from_line(&line_cleaned);
            raw_properties.push(raw_property);
        }

        raw_properties

    }

}
