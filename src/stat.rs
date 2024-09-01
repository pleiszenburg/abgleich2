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

