use crate::path::Path;

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
