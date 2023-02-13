mod ConnexionInfo;
mod SshRequester;
mod Tester;
mod TukTak;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[async_std::main]
async fn main() {
    // Get ip, port & user from options.xml

    let mut file = File::open("options.xml").expect("Failed to open options.xml");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read options.xml");

    let mut properties = HashMap::new();

    for line in content.lines() {
        if line.contains("<entry") {
            let key = line
                .split("key=")
                .nth(1)
                .unwrap()
                .split("\"")
                .nth(1)
                .unwrap();
            let value = line
                .split("<entry")
                .nth(1)
                .unwrap()
                .split(">")
                .nth(1)
                .unwrap()
                .split("<")
                .nth(0)
                .unwrap();
            properties.insert(key.to_owned(), value.to_owned());
        }
    }

    let user = properties.get("user").unwrap();
    let ip = properties.get("ip").unwrap();
    let port = properties.get("port").unwrap().parse::<i32>().unwrap();

    Tester::test_process_all(user, ip, port).await;
}
