use crate::model;

pub trait Root {
    fn print_version(&self);
}

impl Root for model::Root{
    fn print_version(&self) {
        println!("{}", self.version)
    }
}