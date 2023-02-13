#[derive(Clone, Debug)]
pub struct ConnexionInfo<'a> {
    user: &'a str,
    ip: &'a str,
    port: i32,
}

impl<'a> ConnexionInfo<'a> {
    pub fn new(user: &'a str, ip: &'a str, port: i32) -> ConnexionInfo<'a> {
        ConnexionInfo { user, ip, port }
    }
    fn _debug_dump(&self) {
        println!("{:?}", self);
    }
    fn _empty() -> ConnexionInfo<'a> {
        ConnexionInfo {
            user: "",
            ip: "",
            port: -1,
        }
    }

    pub fn ssh_format(&self) -> String {
        format!("ssh://{}@{}:{}", self.user, self.ip, self.port)
    }
}
