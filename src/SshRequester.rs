use crate::ConnexionInfo::ConnexionInfo;
use openssh::{KnownHosts, Session};
use std::io::{self};

pub struct Requester<'a> {
    session: Option<Session>,
    curr_connexion_info: Option<&'a ConnexionInfo<'a>>,
}

// Implementation of async Requester
impl<'a> Requester<'a> {
    pub fn new() -> Requester<'a> {
        Requester {
            session: None,
            curr_connexion_info: None,
        }
    }
    pub async fn connect(&mut self, infos: &'a ConnexionInfo<'a>) -> Result<(), io::Error> {
        let res = Session::connect(infos.ssh_format(), KnownHosts::Strict).await;
        match res {
            Err(_e) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Session creation failed",
                ))
            }
            Ok(sess) => {
                self.session = Some(sess);
                self.curr_connexion_info = Some(infos);
                Ok(())
            }
        }
    }

    pub async fn make_request(&self, command: &String) -> Result<String, io::Error> {
        match &self.session {
            None => return Err(io::Error::new(io::ErrorKind::Other, "No session")),
            Some(session) => {
                // WARNING : shell function executes the commands from ~ location
                // In other words, session does not keep track of the current directory
                let out = session.shell(command).output().await;
                match out {
                    Err(_e) => return Err(io::Error::new(io::ErrorKind::Other, "No session")),
                    Ok(outt) => return Ok(String::from_utf8(outt.stdout).unwrap()),
                }
            }
        }
    }

    pub async fn close(&mut self) -> Result<(), io::Error> {
        if let Some(session) = self.session.take() {
            session.close().await.map_err(|_e| {
                io::Error::new(io::ErrorKind::Other, "Error while closing session")
            })?;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No session to close"))
        }
    }
}
