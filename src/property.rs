use crate::misc::parse_onoff;
use crate::origin::Origin;
use crate::rawproperty::RawProperty;

#[derive(Debug)]
pub struct Property<T> {

    pub value: Option<T>,
    pub origin: Option<Origin>,

}

impl<T> Property<T> {

    pub fn from_empty() -> Property<T> {
        Property {
            value: None,
            origin: None,
        }
    }

}

impl Property<bool> {

    pub fn fill(&mut self, raw_property: &RawProperty) {
        self.value = Some(parse_onoff(&raw_property.value));
        self.origin = Some(Origin::from_raw(&raw_property.meta));
    }

}

impl Property<u64> {

    pub fn fill(&mut self, raw_property: &RawProperty) {
        let result = raw_property.value.parse::<u64>();
        match result {
            Ok(number) => {
                self.value = Some(number);
            }
            Err(error) => {
                panic!("u64 parser fail on {:?} with {:?}", raw_property, error);
            }
        }
        self.origin = Some(Origin::from_raw(&raw_property.meta));
    }

}

impl Property<String> {

    pub fn fill(&mut self, raw_property: &RawProperty) {
        self.value = Some(raw_property.value.clone());
        self.origin = Some(Origin::from_raw(&raw_property.meta));
    }

}
