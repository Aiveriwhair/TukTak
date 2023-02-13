use crate::ConnexionInfo::ConnexionInfo;
use crate::TukTak::TukTak;

fn treat_output(output: String) {
    println!("{}", output);
}

pub async fn test_process_all_async(user: &str, ip: &str, port: i32) {
    let con = ConnexionInfo::new(user, ip, port);
    let connexions = [con.clone(), con.clone(), con.clone(), con.clone(), con];
    let tt = TukTak::new(&connexions);
    let commands = vec![String::from("ls"), String::from("ls ./domains")];
    let res = tt.process_all_async(&commands, treat_output).await;
    match res {
        Err(e) => println!("{}", e),
        _ => (),
    }
}
