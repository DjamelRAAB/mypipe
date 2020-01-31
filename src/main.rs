extern crate clap;
use clap::{Arg, App};
use std::process::Command;
use std::process::Stdio;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;

fn main() {
    let matches = App::new("mypipe").version("0.1.0").author("DjamelRAAB")
    .arg(Arg::with_name("cmd1")
    .long("in")
    .takes_value(true)
    .required(true))
    .arg(Arg::with_name("arg1")
    .takes_value(true)
    .required(false))
    .arg(Arg::with_name("cmd2")
    .long("out")
    .takes_value(true)
    .required(true))
    .arg(Arg::with_name("arg2")
    .takes_value(true)
    .required(false))
    .get_matches()
    ;



    let cmd1 = matches.value_of("cmd1").unwrap();
    let cmd2 = matches.value_of("cmd2").unwrap();
    let arg1 = matches.value_of("arg1").unwrap();
    let arg2 = matches.value_of("arg2").unwrap();

    let cmd1_piped = Command::new(cmd1.to_string())
                        .arg(arg1.to_string())
                        .stdout(Stdio::piped())
                        .spawn();

    unsafe{ 

        let cmd2_output = Command::new(cmd2.to_string())
                            .arg(arg2.to_string())
                            .stdin(Stdio::from_raw_fd(cmd1_piped.ok().unwrap().stdout.unwrap().as_raw_fd()))
                            .output()
                            .expect("Err");

        println!("{}", String::from_utf8_lossy(&cmd2_output.stdout).to_string());
    }
}

