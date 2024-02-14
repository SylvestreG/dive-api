use crate::error::DiveErr;
use crate::modules::users;
use crate::modules::users::services;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenvy::dotenv;
use r2d2::Pool;
use test_context::{test_context, TestContext};

struct LevelTests {
    svc: services::level::Service,
}

impl TestContext for LevelTests {
    fn setup() -> LevelTests {
        dotenv().unwrap();

        let db_url = std::env::var("DATABASE_URL").unwrap();
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool = Pool::builder().build(manager).unwrap();

        LevelTests {
            svc: services::level::Service::new(pool),
        }
    }

    fn teardown(self) {
        // Perform any teardown you wish.
    }
}

#[test_context(LevelTests)]
#[test]
fn list_all_level(ctx: &mut LevelTests) -> Result<(), DiveErr> {
    let mut levels = ctx.svc.list_all_levels()?;

    levels.sort();
    insta::assert_json_snapshot!(levels, {
        "[].created_at" => "[date]",
        "[].updated_at" => "[date]",
    });
    Ok(())
}

#[test_context(LevelTests)]
#[test]
fn get_level(ctx: &mut LevelTests) -> Result<(), DiveErr> {
    let lvl_1 = ctx
        .svc
        .find_by_uuid(users::fixtures::user::DATA[1].id.unwrap())?;
    let lvl_2 = ctx
        .svc
        .find_by_uuid(users::fixtures::user::DATA[1].id.unwrap())?;

    insta::assert_json_snapshot!(lvl_1, {
        ".created_at" => "[date]",
        ".updated_at" => "[date]",
    });
    insta::assert_json_snapshot!(lvl_2, {
        ".created_at" => "[date]",
        ".updated_at" => "[date]",
    });
    Ok(())
}
