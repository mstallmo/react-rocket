use rocket::config::{Config, Environment};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;
use std::io;
use std::process::Command;

pub enum CliCommand {
    NPM,
    YARN,
}

pub struct Engine {
    command: &'static str,
    current_dir: &'static str,
    arg: &'static str,
}

impl Engine {
    pub fn new(command: CliCommand) -> Engine {
        match command {
            CliCommand::NPM => Engine {
                command: "npm",
                current_dir: "./app",
                arg: "start",
            },
            CliCommand::YARN => Engine {
                command: "yarn",
                current_dir: "./app",
                arg: "start",
            },
        }
    }

    fn get_arg_config(config: &Config) -> Option<String> {
        match config.get_str("igniter_arg") {
            Ok(v) => Some(v.to_string()),
            Err(_) => None,
        }
    }

    pub fn run_command(&self, config: &Config) -> Result<&'static str, io::Error> {
        match config.environment {
            Environment::Development => {
                let arg = match Engine::get_arg_config(config) {
                    Some(v) => v,
                    None => self.arg.to_string(),
                };

                Command::new(self.command)
                    .current_dir(self.current_dir)
                    .arg("run")
                    .arg(arg)
                    .spawn()?;
                Ok("🔥  All engines running!")
            }
            _ => Ok("  Not in Development...Skipping"),
        }
    }
}

impl Fairing for Engine {
    fn info(&self) -> Info {
        Info {
            name: "Igniter",
            kind: Kind::Launch,
        }
    }

    fn on_launch(&self, rocket: &Rocket) {
        info!("💨  Ignition sequence start...");
        match self.run_command(rocket.config()) {
            Ok(v) => info!("{}", v),
            Err(e) => warn!("{}", e)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_do_nothing_outside_of_development() {
        let engine = Engine::new(CliCommand::NPM);
        let status = engine.run_command(&Config::new(Environment::Production).expect("cwd"));

        assert!(status.is_ok());
        match status {
            Ok(v) => assert_eq!(v, "  Not in Development...Skipping"),
            _ => ()
        };
    }

    #[test]
    fn it_should_return_err_on_bad_configuration() {
        let broken_engine = Engine {
            command: "not-npm",
            current_dir: "./app",
            arg: "start",
        };
        let status = broken_engine.run_command(&Config::new(Environment::Development).expect("cwd"));
        assert!(status.is_err());
    }
    #[test]
    fn it_should_parse_arg_from_rocket_toml() {
        let rocket = rocket::ignite();
        let igniter_config = Engine::get_arg_config(rocket.config());
        assert!(igniter_config.is_some());
        match igniter_config {
            Some(v) => assert_eq!(v, "test_arg_config"),
            None => (),
        }
    }
}
