use base64::{prelude::BASE64_STANDARD, Engine};
use rocket::{fairing::{Fairing, Info, Kind}, http::{Header, Status}, request::{FromRequest, Outcome}, Request, Response, State};

use crate::models::AuthState;

#[derive(Debug, Clone, thiserror::Error)]
pub enum AuthParseError {
    #[error("Invalid format for auth string, expected username:password")]
    InvalidFormat,
    #[error("Auth password exceeds max length of 255")]
    PasswordTooLong,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Auth {
    pub username: String,
    pub password: String,
}
impl Auth {
    pub fn from(s: &str) -> Result<Auth, AuthParseError> {
        let (user, pass) = s.split_once(':').ok_or(AuthParseError::InvalidFormat)?;

        if pass.len() > 255 {
            Err(AuthParseError::PasswordTooLong)
        } else {
            Ok(Auth {
                username: user.to_owned(),
                password: pass.to_owned(),
            })
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(header) = req.headers().get_one("Authorization") {
            if let Some(auth) = header.strip_prefix("Basic ") {
                if let Ok(decoded) = BASE64_STANDARD.decode(auth) {
                    if let Ok(credentials) = String::from_utf8(decoded) {
                        if let Ok(auth) = Self::from(&credentials) {
                            return Outcome::Success(auth);
                        }
                    }
                }
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}

pub struct AuthGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ();
    
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auths = match req.guard::<&State<AuthState>>().await {
            Outcome::Success(auths) => &auths.auths,
            _ => return Outcome::Error((Status::InternalServerError, ())),
        };

        if auths.len() > 0 {
            let auth = match req.guard::<Auth>().await {
                Outcome::Success(auth) => auth,
                _ => {
                    return Outcome::Error((Status::Unauthorized, ()))
                }
            };
            if auths.contains(&auth) {
                Outcome::Success(AuthGuard)
            } else {
                Outcome::Error((Status::Unauthorized, ()))
            }
        } else {
            Outcome::Success(AuthGuard)
        }
    }
}

pub struct AuthFairing;

#[rocket::async_trait]
impl Fairing for AuthFairing {
    fn info(&self) -> Info {
        Info {
            name: "Add WWW-Authenticate Header on Unauthorized",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        if res.status() == Status::Unauthorized {
            res.set_header(Header::new(
                "WWW-Authenticate",
                r#"Basic realm="Restricted Area""#,
            ));
        }
    }
}
 