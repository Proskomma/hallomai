use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub struct Element {
    pub tag_name: String,
    pub attributes: BTreeMap<String, String>,
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ELEMENT<{}>, {:#?}", self.tag_name, self.attributes)
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ELEMENT<{}>, {:#?}", self.tag_name, self.attributes)
    }
}
