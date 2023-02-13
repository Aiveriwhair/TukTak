#[derive(Clone, Debug)]
pub struct ConnexionInfo {
    user: String,
    ip: String,
    port: i32,
}

impl ConnexionInfo {
    pub fn new(user: String, ip: String, port: i32) -> ConnexionInfo {
        ConnexionInfo { user, ip, port }
    }
    fn _debug_dump(&self) {
        println!("{:?}", self);
    }
    fn _empty() -> ConnexionInfo {
        ConnexionInfo {
            user: "".to_string(),
            ip: "".to_string(),
            port: -1,
        }
    }

    pub fn ssh_format(&self) -> String {
        format!("ssh://{}@{}:{}", self.user, self.ip, self.port)
    }
}
