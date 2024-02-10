use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use r2d2::Pool;
use test_context::{test_context, TestContext};
use crate::error::DiveErr;
use crate::modules::users::services;

struct UserTests {
    svc: services::user::Service,
}

impl TestContext for UserTests {
    fn setup() -> UserTests {
        dotenv().unwrap();

        let db_url = std::env::var("DATABASE_URL").unwrap();
        let manager =
            ConnectionManager::<PgConnection>::new(db_url);
        let pool = Pool::builder().build(manager).unwrap();


        UserTests { svc: services::user::Service::new(pool)}
    }

    fn teardown(self) {
        // Perform any teardown you wish.
    }
}

#[test_context(UserTests)]
#[test]
fn list_all_user(ctx: &mut UserTests) -> Result<(), DiveErr>{
    let mut users = ctx.svc.list_all_user()?;

    users.sort();
    insta::assert_json_snapshot!(users, {
        "[].id" => "[uuid]",
        "[].created_at" => "[date]",
        "[].updated_at" => "[date]",
    });
    Ok(())
}
