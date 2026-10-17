use std::io::{Result, Write, stdin, stdout};
use std::process::{Child, Command, Output, Stdio};

#[derive(Debug, PartialEq)]
struct ShellCommand<'a> {
    cmd: &'a str,
    args: Vec<&'a str>,
}

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

        if input.trim() == "exit" {
            break;
        }

        let commands = parse_pipeline(&input);

        if commands.is_empty() {
            continue;
        }

        match run_pipeline(commands) {
            Ok(exit_status) => {
                if exit_status.status.success() {
                    stdout()
                        .write_all(&exit_status.stdout)
                        .expect("Failed to write to stdout");
                } else {
                    eprintln!("Process exited: {}", exit_status.status);
                }
            }
            Err(error) => eprintln!("Failed to execute: {}", error),
        }
    }
}

fn parse_pipeline(input: &str) -> Vec<ShellCommand<'_>> {
    input
        .split("|")
        .filter_map(|part| get_command_with_args(part))
        .collect()
}

fn get_command_with_args(s: &str) -> Option<ShellCommand<'_>> {
    let mut arr = s.split_whitespace();

    Some(ShellCommand {
        cmd: arr.next()?,
        args: arr.collect(),
    })
}

fn run_pipeline(commands: Vec<ShellCommand>) -> Result<Output> {
    let first_child = Command::new(commands[0].cmd)
        .args(&commands[0].args)
        .stdout(Stdio::piped())
        .spawn();

    commands
        .iter()
        .skip(1)
        .fold(first_child, |child, item| get_child(child, item))?
        .wait_with_output()
}

fn get_child(prev: Result<Child>, sc: &ShellCommand) -> Result<Child> {
    Command::new(sc.cmd)
        .args(&sc.args)
        .stdin(Stdio::from(prev?.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::process::Child;
    use std::process::{Command, Stdio};

    #[test]
    fn test_get_command_with_args() {
        assert_eq!(get_command_with_args(""), None);
        assert_eq!(
            get_command_with_args("exit"),
            Some(ShellCommand {
                cmd: "exit",
                args: vec![]
            })
        );
        assert_eq!(
            get_command_with_args("ls -la"),
            Some(ShellCommand {
                cmd: "ls",
                args: vec!["-la"]
            })
        );
        assert_eq!(
            get_command_with_args("ls -la /tmp"),
            Some(ShellCommand {
                cmd: "ls",
                args: vec!["-la", "/tmp"]
            })
        );
        assert_eq!(
            get_command_with_args("echo Hello"),
            Some(ShellCommand {
                cmd: "echo",
                args: vec!["Hello"]
            })
        );
        assert_eq!(
            get_command_with_args("echo Hello World"),
            Some(ShellCommand {
                cmd: "echo",
                args: vec!["Hello", "World"]
            })
        );
        assert_eq!(
            get_command_with_args("pwd /home/user/project"),
            Some(ShellCommand {
                cmd: "pwd",
                args: vec!["/home/user/project"]
            })
        );
    }

    #[test]
    fn test_get_child_success() -> Result<()> {
        let prev = Command::new("printf")
            .arg("hello rust\n")
            .stdout(Stdio::piped())
            .spawn()?;

        let sc = ShellCommand {
            cmd: "grep",
            args: vec!["rust"],
        };

        let child = get_child(Ok(prev), &sc)?;
        let output = child.wait_with_output()?;

        assert!(output.status.success());
        assert_eq!(output.stdout, b"hello rust\n");

        Ok(())
    }

    #[test]
    fn test_get_child_no_matches() -> Result<()> {
        let prev = Command::new("printf")
            .arg("hello rust\n")
            .stdout(Stdio::piped())
            .spawn()?;

        let sc = ShellCommand {
            cmd: "grep",
            args: vec!["java"],
        };

        let child = get_child(Ok(prev), &sc)?;
        let output = child.wait_with_output()?;

        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());

        Ok(())
    }

    #[test]
    fn test_get_child_invalid_command() -> Result<()> {
        let prev = Command::new("printf")
            .arg("hello\n")
            .stdout(Stdio::piped())
            .spawn()?;

        let sc = ShellCommand {
            cmd: "nonexistent_command_123456789",
            args: vec![],
        };

        let result = get_child(Ok(prev), &sc);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);

        Ok(())
    }

    #[test]
    fn test_get_child_previous_error() {
        let prev: Result<Child> = Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Previous process failed",
        ));

        let sc = ShellCommand {
            cmd: "grep",
            args: vec!["rust"],
        };

        let result = get_child(prev, &sc);

        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::PermissionDenied);
    }
}
