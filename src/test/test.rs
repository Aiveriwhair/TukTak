use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

use crate::tuktak::ConnexionInfo::ConnexionInfo;
use crate::tuktak::TukTak::TukTak;

fn treat_output(output: String) {
    write_output(output);
}

fn write_output(output: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("output.log")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", output) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn load_config<'a>(connexions_path: &str, commands_path: &str) -> (TukTak, Vec<String>) {
    // Create the connexions vector and commands vector
    let mut connexions = Vec::new();
    let mut commands = Vec::new();

    if let Ok(lines) = read_lines(connexions_path.to_string()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(infos) = line {
                let mut split = infos.split_whitespace();
                let user = split.next().unwrap();
                let ip = split.next().unwrap();
                let port = split.next().unwrap().parse::<i32>().unwrap();
                let con = ConnexionInfo::new(user.to_string(), ip.to_string(), port);
                connexions.push(con);
            }
        }
    }
    if let Ok(lines) = read_lines(commands_path.to_string()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(command) = line {
                commands.push(command);
            }
        }
    }
    let tt = TukTak::new(connexions);
    return (tt, commands);
}

pub async fn test_process_all_async(test_number: usize) {
    let connexions_path = "./tests_config/connexions/".to_string() + &test_number.to_string();
    let commands_path = "./tests_config/commands/".to_string() + &test_number.to_string();
    let (tt, commands) = load_config(&connexions_path, &commands_path);

    let res = tt.process_all_async(&commands, treat_output).await;
    match res {
        Err(e) => println!("{}", e),
        _ => (),
    }
}
