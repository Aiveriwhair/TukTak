use crate::ConnexionInfo::ConnexionInfo;
use crate::TukTak::TukTak;

fn treat_output(output: String) {
    println!("{}", output);
}

pub async fn test_process_all(user: &str, ip: &str, port: i32) {
    let con = ConnexionInfo::new(user, ip, port);
    let connexions = [con.clone(), con.clone(), con];
    let tt = TukTak::new(&connexions);
    let res = tt.process_all(String::from("ls"), treat_output).await;
    match res {
        Err(e) => eprintln!("{}", e),
        _ => (),
    }
}
