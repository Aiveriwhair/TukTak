use crate::ConnexionInfo::ConnexionInfo;
use crate::SshRequester::Requester;
use futures::future::join_all;
use std::io;

pub struct TukTak<'a> {
    user_ip_port: &'a [ConnexionInfo<'a>],
}
impl<'a> TukTak<'a> {
    pub fn new(connexions: &'a [ConnexionInfo]) -> TukTak<'a> {
        TukTak {
            user_ip_port: connexions,
        }
    }
}

// TukTak threaded
impl<'a> TukTak<'a> {
    pub fn _process_all_threaded(
        &self,
        _command: String,
        _callback: fn(String) -> (),
        _th_num: usize,
    ) -> Result<(), io::Error> {
        todo!()
    }
}

// TukTak with async functions, not threaded
impl<'a> TukTak<'a> {
    pub async fn process(
        &self,
        commands: Vec<String>,
        connexion: &ConnexionInfo<'a>,
        callback: fn(String) -> (),
    ) -> Result<(), io::Error> {
        let mut requester = Requester::new();
        let res_con = requester.connect(connexion).await;
        match res_con {
            Err(e) => return Err(e),
            _ => (),
        }

        for command in commands {
            let output = requester.make_request(&command).await;
            callback(format!(
                "{:?}> {} > {}",
                connexion,
                command,
                output.unwrap()
            ));
        }

        let res_close = requester.close().await;
        match res_close {
            Err(e) => return Err(e),
            _ => return Ok(()),
        }
    }

    // Process all the connexions in the array with the given command
    // NO THREADS
    // Returns time to process all the connexions
    pub async fn process_all_async(
        &self,
        commands: &Vec<String>,
        callback: fn(String) -> (),
    ) -> Result<(), io::Error> {
        let mut i: usize = 1;
        let start_time = std::time::Instant::now();

        // Create a vector of futures
        let mut futures = Vec::new();

        for connexion in self.user_ip_port.iter() {
            let future = self.process(commands.clone(), connexion, callback);
            futures.push(future);

            i += 1;
        }
        // Join all the futures
        join_all(futures).await;

        let end_time = std::time::Instant::now();
        let duration = end_time.duration_since(start_time);
        println!(
            "Time elapsed in process_all_async() for {} connexions is: {:?}",
            self.user_ip_port.len(),
            duration
        );
        Ok(())
    }
}
