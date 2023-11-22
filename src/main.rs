
use std::{env::args, process::{exit, Command}, fmt::format};
fn main() {
    let args = args().skip(1).collect::<Vec<String>>();
    if args.len() < 3{
        println!("Format is:-\n program [token] [protected-branch-name] [repo] \nCurrent arguments are:{:#?}",args);

        eprintln!("Not enough arguments provided! Exiting");
        
        exit(-1)
    }
    let token = args.get(0).take().unwrap();
    let branch_name = args.get(1).take().unwrap();
    let repo = args.get(2).take().unwrap().strip_prefix("https://github.com/").take().unwrap().strip_suffix(".git").take().unwrap().to_string();

    let cmd = format!("curl -X DELETE -H \"Authorization: token {}\" \
https://api.github.com/{}/branches/{}/protection",token,repo,branch_name);


    use std::process::Command;


    let mut command = Command::new("curl").args(["-X","DELETE","-H",&format!("\"Authorization: token {}\"",token),&format!(" \\ https://api.github.com/{}/branches/{}/protection",repo,branch_name)])
    .output().expect("Failed to run curl");

    let output = String::from_utf8(command.stdout).unwrap();

    println!("{}",output);


    println!("{}",cmd);

    println!("{:#?}",args);


}