#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {

    loop {
        print!("% ");
        io::stdout().flush().unwrap();
    
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        //print if command not found
        let command = input.trim();

        // place inputted commands seperated by whitespaces into parts vector
        let parts: Vec<&str> = command.split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];

        // check if type command inputted
        if parts.len() > 1 && parts[0] == "type" {

            if parts[1] == "echo" || parts[1] == "exit" || parts[1] == "type" {
                println!("{} is a shell builtin", parts[1]);
            } else {

                // retrieve PATH env variable
                // split into directories vector
                let path_var = env::var("PATH").unwrap_or_default();
                let paths: Vec<&str> = path_var.split(':').collect();
                let mut found = false;
                
                // iterate over each directory in PATH
                for path in paths {

                    // contruct full path
                    let mut full_path = Path::new(path).join(parts[1]);
                    full_path.set_extension(""); // remove extension 

                    // check if path exists and print full path
                    if full_path.exists() {
                        println!("{} is {}",parts[1], full_path.display());
                        found = true;
                        break;
                    } 
                }

                // print if command not found in path
                if !found {
                    println!("{}: not found", parts[1]);
                    io::stdout().flush().unwrap();
                }
            } 
        } else if parts.len() > 1 && parts[0] == "echo" {
            
            // join echo argugments with spaces
            // echo arguements
            let output = parts[1..].join(" ");
            println!("{}", output);

        // if exit command inputted, exit REPL
        } else if command == "exit" && parts[1] == "0" {
            process::exit(0);

        } else if command == "cd" {
            
            // if parts vector greater than 1
            // change current directory
            if parts.len() > 1 {
                let new_dir = parts[1];
                if let Err(e) = env::set_current_dir(new_dir) {
                    eprintln!("cd: {}: {}", new_dir, e);
                }

            } else {
                println!("cd: missing operand");
            }

        } else if command == "pwd" {
            
            match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("pwd: {}", e),
            }

        } else {

            let path_var = env::var("PATH").unwrap_or_default();
            let paths: Vec<&str> = path_var.split(':').collect();
            let mut executable_path = None;

            for path in paths {
                let full_path = Path::new(path).join(command);

                if full_path.exists() {
                    executable_path = Some(full_path);
                    break;
                }
            }

            if let Some(executable_path) = executable_path {
                let mut command = Command::new(executable_path);
                command.args(args);

                match command.spawn() {
                    Ok(mut child) => {
                        let _ = child.wait();
                    },
                    Err(e) => {
                        eprintln!("Failed to execute command: {}", e);
                    }
                }
            } else {
                println!("{}: command not found", command);
            }
        }
    }     
}

