use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request};

pub struct AuthenticationFairing;

#[rocket::async_trait]
impl Fairing for AuthenticationFairing {
    fn info(&self) -> Info {
        Info {
            name: "JWT Authenticator",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, _req: &mut Request<'_>, _data: &mut Data<'_>) {
        dbg!(_req);
        // dbg!(_data);
        todo!()
    }
}
