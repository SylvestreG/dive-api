use crate::error::DiveErr;
use crate::modules::core::fixtures;
use crate::modules::users::models::level::New;
use crate::modules::users::repositories;

pub const DATA: &[New] = &[
    New {
        id: Some(uuid::Uuid::from_u64_pair(
            0xf339_cf72_47c5_48f8u64,
            0x9ad1_ff1d_93f6_c2e6u64,
        )),
        level_name: "Niveau 1",
    },
    New {
        id: Some(uuid::Uuid::from_u64_pair(
            0xf633_eb82_ff98_4e93u64,
            0x9264_a27c_1bb5_4c87u64,
        )),
        level_name: "Niveau 2",
    },
    New {
        id: Some(uuid::Uuid::from_u64_pair(
            0x5232_de05_4495_4fecu64,
            0xa45f_18bb_ef3f_8ec3u64,
        )),
        level_name: "Niveau 3",
    },
    New {
        id: Some(uuid::Uuid::from_u64_pair(
            0x8886_07ea_887f_415au64,
            0x89cd_20df_3153_e009u64,
        )),
        level_name: "Niveau 4",
    },
    New {
        id: Some(uuid::Uuid::from_u64_pair(
            0x8886_07ea_887f_416au64,
            0x89cd_20df_3153_e009u64,
        )),
        level_name: "Moniteur Fédéral 1",
    },
    New {
        id: Some(uuid::Uuid::from_u64_pair(
            0x8886_07ea_887f_417au64,
            0x89cd_20df_3153_e009u64,
        )),
        level_name: "Moniteur Fédéral 2",
    },
];

pub struct Fixtures {
    repository: repositories::level::Repository,
}

impl Fixtures {
    #[must_use]
    pub fn new(repository: repositories::level::Repository) -> Self {
        Fixtures { repository }
    }
}

impl fixtures::Trait for Fixtures {
    fn load(&mut self) -> Result<(), DiveErr> {
        for entry in DATA {
            self.repository.new_level(entry)?;
        }
        Ok(())
    }

    fn delete(&mut self) -> Result<(), DiveErr> {
        self.repository.drop_all()
    }
}
