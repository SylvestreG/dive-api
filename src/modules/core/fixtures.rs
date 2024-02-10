use crate::error::DiveErr;

pub trait Trait {

    /// # Errors
    ///
    /// Will return `DiveErr` if the sql does not work
    /// during the fixture load
    fn load(&mut self) -> Result<(), DiveErr>;

    /// # Errors
    ///
    /// Will return `DiveErr` if the sql does not work
    /// during the fixture load
    fn delete(&mut self) -> Result<(), DiveErr>;
}
