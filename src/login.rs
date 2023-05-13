use std::{env, os::unix::net::UnixStream};

use greetd_ipc::{codec::SyncCodec, AuthMessageType, ErrorType, Request, Response};

pub enum LoginResult {
    Success,
    Failure(String),
}

pub fn login(
    username: String,
    passward: String,
    command: Vec<String>,
) -> anyhow::Result<LoginResult> {
    let mut stream = UnixStream::connect(env::var("GREETD_SOCK")?)?;

    let mut next_request = Request::CreateSession { username };
    let mut starting = false;
    loop {
        next_request.write_to(&mut stream)?;
        match Response::read_from(&mut stream)? {
            Response::AuthMessage {
                auth_message_type,
                auth_message,
            } => {
                let response = match auth_message_type {
                    AuthMessageType::Secret => Some(passward.clone()),
                    AuthMessageType::Error => {
                        eprintln!("Error: {auth_message}");
                        None
                    }
                    _ => None,
                };
                next_request = Request::PostAuthMessageResponse { response };
            }
            Response::Success => {
                if starting {
                    return Ok(LoginResult::Success);
                } else {
                    starting = true;
                    next_request = Request::StartSession {
                        cmd: command.clone(),
                        env: vec![],
                    }
                }
            }
            Response::Error {
                error_type,
                description,
            } => {
                Request::CancelSession.write_to(&mut stream)?;
                match error_type {
                    ErrorType::AuthError => return Ok(LoginResult::Failure(description)),
                    ErrorType::Error => {
                        return Ok(LoginResult::Failure(
                            format!("login error: {:?}", description).into(),
                        ))
                    }
                }
            }
        }
    }
}
