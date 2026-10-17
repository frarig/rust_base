use std::io::{Result, Write, stdin, stdout};
use std::process::{Command, ExitStatus};

pub fn run() {
    let mut input = String::new();

    loop {
        print!("blazing_sell> ");
        stdout().flush().expect("Failed to flush stdout");

        input.clear();
        let bytes_read = stdin().read_line(&mut input).expect("Failed to read stdin");

        if bytes_read == 0 {
            break;
        }

        let Some((cmd, args)) = get_command_with_args(&input) else {
            continue;
        };

        if cmd == "exit" {
            break;
        }

        match command_run(cmd, args) {
            Ok(status) => {
                if !status.success() {
                    eprintln!("Process exited with status: {}", status);
                }
            }
            Err(error) => eprintln!("Failed to execute {} : {}", cmd, error),
        }
    }
}

fn get_command_with_args(s: &str) -> Option<(&str, Vec<&str>)> {
    let mut arr = s.split_whitespace();
    let cmd = arr.next()?;
    let args = arr.collect();

    Some((cmd, args))
}

fn command_run(cmd: &str, args: Vec<&str>) -> Result<ExitStatus> {
    Command::new(cmd).args(args).status()
}

#[test]
fn test_get_command_with_args() {
    assert_eq!(get_command_with_args(""), None);
    assert_eq!(get_command_with_args("exit"), Some(("exit", vec![])));
    assert_eq!(get_command_with_args("ls -la"), Some(("ls", vec!["-la"])));
    assert_eq!(
        get_command_with_args("ls -la /tmp"),
        Some(("ls", vec!["-la", "/tmp"]))
    );
    assert_eq!(
        get_command_with_args("echo Hello"),
        Some(("echo", vec!["Hello"]))
    );
    assert_eq!(
        get_command_with_args("echo Hello World"),
        Some(("echo", vec!["Hello", "World"]))
    );
    assert_eq!(
        get_command_with_args("pwd /home/user/project"),
        Some(("pwd", vec!["/home/user/project"]))
    );
}

#[test]
fn test_command_success() {
    let result = command_run("true", vec![]);
    assert!(result.is_ok());

    let status = result.unwrap();
    assert!(status.success());
    assert_eq!(status.code(), Some(0));
}

#[test]
fn test_command_failure() {
    let result = command_run("false", vec![]);
    assert!(result.is_ok());

    let status = result.unwrap();
    assert!(!status.success());
    assert_eq!(status.code(), Some(1));
}

#[test]
fn test_command_not_found() {
    let result = command_run("hehesdx", vec![]);

    assert!(result.is_err());
}
