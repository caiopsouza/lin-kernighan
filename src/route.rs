use crate::path::Path;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub struct Route {
    pub cost: u32,
    pub path: Path,
}

impl Route
{
    pub fn new(cost: u32, path: Path) -> Route {
        Route { cost, path }
    }
}

impl Display for Route {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Route {{ cost: {}, {} }}", self.cost, self.path)
    }
}
