use rocket;
use fair_teams;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    fair_teams::rocket().await.launch().await?;

    Ok(())
}
