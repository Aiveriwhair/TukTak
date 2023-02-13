use crate::ConnexionInfo::ConnexionInfo;
use crate::SshRequester::Requester;
use std::io::{self};

pub struct TukTak<'a> {
    user_ip_port: &'a [ConnexionInfo<'a>],
}
impl<'a> TukTak<'a> {
    pub fn new(connexions: &'a [ConnexionInfo]) -> TukTak<'a> {
        TukTak {
            user_ip_port: connexions,
        }
    }
    fn _debug_dump(&self) {
        for connexion in self.user_ip_port.iter() {
            println!("{:?}", connexion);
        }
    }
}

impl<'a> TukTak<'a> {
    pub fn _fast_process_all(
        &self,
        _command: String,
        _callback: fn(String) -> (),
        _th_num: usize,
    ) -> Result<(), io::Error> {
        todo!()
    }
}

impl<'a> TukTak<'a> {
    // Process a single connexion with the given command
    // NO THREADS
    pub async fn process(
        &self,
        command: String,
        connexion: &ConnexionInfo<'a>,
        callback: fn(String) -> (),
    ) -> Result<(), io::Error> {
        let mut requester = Requester::new();
        let res_con = requester.connect(connexion).await;
        match res_con {
            Err(e) => return Err(e),
            _ => (),
        }

        let output = requester.make_request(&command).await;
        callback(output.unwrap());
        let res_close = requester.close().await;
        match res_close {
            Err(e) => return Err(e),
            _ => return Ok(()),
        }
    }

    // Process all the connexions in the array with the given command
    // NO THREADS
    // Returns time to process all the connexions
    pub async fn process_all(
        &self,
        command: String,
        callback: fn(String) -> (),
    ) -> Result<(), io::Error> {
        let mut i: usize = 1;
        let start_time = std::time::Instant::now();
        for connexion in self.user_ip_port.iter() {
            println!(
                "###############\nProcessing command:\n{}\n> {:?}\n###############\n",
                command.clone(),
                connexion
            );

            self.process(command.clone(), connexion, callback).await?;

            println!("{} / {}", i, self.user_ip_port.len());
            println!("---  DONE  ---");
            i += 1;
        }
        let end_time = std::time::Instant::now();
        let duration = end_time.duration_since(start_time);
        println!(
            "Time elapsed in process_all() for {} connexions is: {:?}",
            self.user_ip_port.len(),
            duration
        );
        Ok(())
    }
}
