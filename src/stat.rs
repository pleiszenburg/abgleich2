use crate::datasettype::DatasetType;
use crate::rawproperty::RawProperty;
use crate::misc::parse_yesno;

#[derive(Debug)]
pub struct Stat<T> {

    pub value: Option<T>,

}

impl<T> Stat<T> {

    pub fn from_empty () -> Stat<T> {
        Stat{
            value: None,
        }
    }

}

impl Stat<DatasetType> {

    pub fn fill(&mut self, raw_property: &RawProperty) {
        self.value = Some(DatasetType::from_raw(&raw_property.value));
    }

}

impl Stat<bool> {

    pub fn fill(&mut self, raw_property: &RawProperty) {
        self.value = Some(parse_yesno(&raw_property.value));
    }

}

impl Stat<f32> {

    pub fn fill(&mut self, raw_property: &RawProperty) {
        let result = raw_property.value.parse::<f32>();
        match result {
            Ok(number) => {
                self.value = Some(number);
            }
            Err(error) => {
                panic!("f32 parser fail on {:?} with {:?}", raw_property, error);
            }
        }
    }

}

impl Stat<u64> {

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
    }

}
