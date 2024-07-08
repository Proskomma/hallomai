#![allow(dead_code)]
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

/// # Represents an XML element with a tag name and attributes.
#[derive(Clone)]
pub struct Element {
    /// The tag name of the element.
    pub tag_name: String,
    /// A map of attribute names to their values.
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
