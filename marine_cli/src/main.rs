mod hint;
mod login;
mod xdginfos;
use rpassword::prompt_password;
use rustyline::Editor;
use rustyline::{error::ReadlineError, history::DefaultHistory};

use std::error::Error;
use std::fs;

use crate::{
    login::{login, LoginResult},
    xdginfos::DESKTOPS,
};
use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;
use hint::*;
const COMMANDS: &[&str] = &["loginwm", "loginshell", "showinfo", "help", "exit", "clear"];
use std::collections::HashSet;

//use rustyline::hint::{Hint, Hinter};
enum RustLineType {
    CommandChoose,
    LoginShell,
    LoginWm,
    ToLogin,
}

fn diy_hints() -> HashSet<CommandHint> {
    let mut set = HashSet::new();
    set.insert(CommandHint::new("help", "help"));
    set.insert(CommandHint::new("loginwm", "loginwm"));
    set.insert(CommandHint::new("loginshell", "loginshell"));
    set.insert(CommandHint::new("showinfo", "showinfo"));
    set.insert(CommandHint::new("clear", "clear"));
    set.insert(CommandHint::new("exit", "exit"));
    set.insert(CommandHint::new("s", "s"));
    set
}

fn hostname() -> String {
    fs::read_to_string("/etc/hostname")
        .unwrap_or(String::new())
        .trim()
        .to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    let h = DIYHinter { hints: diy_hints() };
    let mut rl = Editor::<DIYHinter, DefaultHistory>::new()?;

    let mut command: String = String::new();
    let mut username: String = String::new();
    let mut password: String = String::new();
    let defaultpromot = format!("{} >> ", hostname());
    let mut prompt: &str = &defaultpromot;
    let mut currenttype = RustLineType::CommandChoose;
    loop {
        let readline = if let RustLineType::ToLogin = currenttype {
            rl.readline_with_initial(prompt, (command.as_str(), ""))
        } else {
            rl.readline(prompt)
        };
        if let RustLineType::CommandChoose = currenttype {
            rl.set_helper(Some(h.clone()));
        } else {
            rl.set_helper(None);
        }
        match readline {
            Ok(mut line) => {
                if line == "s" {
                    let chooseindex = choose_command();
                    if chooseindex == -1 {
                        println!("cancel");
                        continue;
                    }
                    line = COMMANDS[chooseindex as usize].to_string();
                }
                match currenttype {
                    RustLineType::CommandChoose => match line.as_str() {
                        "clear" => {
                            println!("{}c", 27 as char);
                        }
                        "loginwm" => {
                            currenttype = RustLineType::LoginWm;
                            prompt = "UserName: ";
                        }
                        "loginshell" => {
                            currenttype = RustLineType::LoginShell;
                            prompt = "UserName: ";
                        }
                        "showinfo" => {
                            let wm_index = choose_wm();
                            if wm_index == -1 {
                                println!("You have not choose a wm");
                                continue;
                            } else {
                                let comment = (&*DESKTOPS)[wm_index as usize]
                                    .comment
                                    .clone()
                                    .unwrap_or_default();
                                println!("{comment}");
                            }
                        }
                        "exit" => break,
                        "help" => {
                            println!("use 'clear' to clear terminal");
                            println!("use 'loginwm' to login the wm");
                            println!("use 'loginshell' to login with the command you want");
                            println!("use 'showinfo' to show the wm info");
                            println!("use 'exit' to exit");
                            println!("use 's' to fuzzle select commands");
                        }
                        _ => println!("no such command"),
                    },
                    RustLineType::LoginWm => {
                        username = line;
                        password = prompt_password("password: ").unwrap();
                        let wm_index = choose_wm();
                        if wm_index == -1 {
                            println!("You have not choose a wm");
                            continue;
                        } else {
                            command = (&*DESKTOPS)[wm_index as usize].exec.clone();
                            currenttype = RustLineType::ToLogin;
                            prompt = "Command:";
                        }
                    }
                    RustLineType::LoginShell => {
                        username = line;
                        password = prompt_password("password: ").unwrap();
                        prompt = "Shell: ";
                        currenttype = RustLineType::ToLogin;
                    }
                    RustLineType::ToLogin => {
                        let cmd = line
                            .split(' ')
                            .collect::<Vec<&str>>()
                            .into_iter()
                            .map(|cmd| cmd.to_string())
                            .collect();
                        match login(username.clone(), password.clone(), cmd) {
                            Ok(LoginResult::Success) => {
                                break;
                            }
                            Ok(LoginResult::Failure(message)) => {
                                println!("Error: {message}");
                                currenttype = RustLineType::CommandChoose;
                                prompt = &defaultpromot;
                                command = String::new();
                            }
                            Err(e) => {
                                println!("Error to Login: {e}");
                                break;
                            }
                        };
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                currenttype = RustLineType::CommandChoose;
                prompt = &defaultpromot;
                command = String::new();
                println!("CTRL-C");
                println!("Cancel to select");
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

fn choose_wm() -> i32 {
    let wms = &*DESKTOPS
        .iter()
        .map(|wm| wm.name.clone())
        .collect::<Vec<String>>();
    let Ok(index) = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Now to choose a wm")
        .default(0)
        .items(wms)
        .interact() else {
        return -1;
    };
    index as i32
}

fn choose_command() -> i32 {
    let Ok(index) = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Now to choose a wm")
        .default(0)
        .items(COMMANDS)
        .interact() else {
        return -1;
    };
    index as i32
}
