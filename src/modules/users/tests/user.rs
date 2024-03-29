use crate::error::DiveErr;
use crate::modules::users;
use crate::modules::users::services;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenvy::dotenv;
use r2d2::Pool;
use test_context::{test_context, TestContext};

struct UserTests {
    svc: services::user::Service,
}

impl TestContext for UserTests {
    fn setup() -> UserTests {
        dotenv().unwrap();

        let db_url = std::env::var("DATABASE_URL").unwrap();
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool = Pool::builder().build(manager).unwrap();

        UserTests {
            svc: services::user::Service::new(pool),
        }
    }

    fn teardown(self) {
        // Perform any teardown you wish.
    }
}

#[test_context(UserTests)]
#[test]
fn list_all_user(ctx: &mut UserTests) -> Result<(), DiveErr> {
    let mut users = ctx.svc.list_all_user()?;

    users.sort();
    insta::assert_json_snapshot!(users, {
        "[].created_at" => "[date]",
        "[].updated_at" => "[date]",
    });
    Ok(())
}

#[test_context(UserTests)]
#[test]
fn get_user(ctx: &mut UserTests) -> Result<(), DiveErr> {
    let user_1 = ctx
        .svc
        .find_by_uuid(users::fixtures::user::DATA[1].id.unwrap())?;
    let user_2 = ctx
        .svc
        .find_by_uuid(users::fixtures::user::DATA[1].id.unwrap())?;

    insta::assert_json_snapshot!(user_1, {
        ".created_at" => "[date]",
        ".updated_at" => "[date]",
    });
    insta::assert_json_snapshot!(user_2, {
        ".created_at" => "[date]",
        ".updated_at" => "[date]",
    });
    Ok(())
}
