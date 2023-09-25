use rocket::http::Status;
use rocket::request::{self, Outcome, Request, FromRequest};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use uuid::{Builder, Variant, Version};
use crate::AppState;
use crate::teams::Player;

#[derive(Debug, Clone)]
pub struct Session {
    pub player: Player,
    pub token: String
}

impl Session {
    pub fn new(player: Player) -> Session {
        let seed = [0u8; 32];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let mut bytes = [0u8; 16];
        rng.fill_bytes(&mut bytes);

        let mut binding = Builder::from_bytes(bytes);
        let uuid = binding
            .set_variant(Variant::RFC4122)
            .set_version(Version::Random)
            .as_uuid();

        Session {
            player,
            token: uuid.to_string()
        }
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for Session {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> request::Outcome<Session, ()> {
        let state = request.rocket().state::<AppState>().unwrap();

        match request.cookies()
            .get_private("token")
            .and_then(|cookie| state.verify_session(cookie.value().to_string()))
            {
                None => {Outcome::Failure((Status::Unauthorized, ()))},
                Some(session) => {Outcome::Success(session)}
            }
    }
}