use std::str::FromStr;
use crate::error::DiveErr;
use crate::modules::core::fixtures;
use crate::modules::users::models::user::New;
use crate::modules::users::repositories;

pub struct Fixtures {
    repository: repositories::user::Repository,
}

impl Fixtures {
    #[must_use]
    pub fn new(repository: repositories::user::Repository) -> Self {
       Fixtures { repository }
    }
}

impl fixtures::Trait for Fixtures {
    fn load(&mut self) -> Result<(), DiveErr> {
        self.repository.new_user(&New {
            id: Some(uuid::Uuid::from_str("f339cf72-47c5-48f8-9ad1-ff1d93f6c2e6")?),
            firstname: "sylvestre",
            lastname: "gallon",
            mail: "gallon_s@epitech.net",
            tel: "0663903521",
        })?;

        self.repository.new_user(&New {
            id: Some(uuid::Uuid::from_str("f633eb82-ff98-4e93-9264-a27c1bb54c87")?),
            firstname: "aurel",
            lastname: "bussierre",
            mail: "aurel@epitech.net",
            tel: "0663903521",
        })?;

        self.repository.new_user(&New {
            id: Some(uuid::Uuid::from_str("5232de05-4495-4fec-a45f-18bbef3f8ec3")?),
            firstname: "alex",
            lastname: "chauvet",
            mail: "alex@epitech.net",
            tel: "0663903521",
        })?;

        self.repository.new_user(&New {
            id: Some(uuid::Uuid::from_str("888607ea-887f-415a-89cd-20df3153e009")?),
            firstname: "willy",
            lastname: "marteau",
            mail: "willy@epitech.net",
            tel: "0663903521",
        })
    }

    fn delete(&mut self) -> Result<(), DiveErr> {
        self.repository.drop_all()
    }
}
