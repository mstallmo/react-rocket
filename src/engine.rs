use rocket::config::Environment;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;
use std::process::Command;
use std::io;

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

    pub fn run_command(&self, environment: Environment) -> Result<&'static str, io::Error> {
        match environment {
            Environment::Development => {
                Command::new(self.command)
                    .current_dir(self.current_dir)
                    .arg(self.arg)
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
        match self.run_command(rocket.config().environment) {
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
        let status = engine.run_command(Environment::Production);

        assert!(status.is_ok());
        match status {
            Ok(v) => assert_eq!(v, "  Not in Development...Skipping"),
            _ => ()
        };
    }

    #[test]
    fn it_should_return_err_on_bad_configuration() {
        let broken_engine = Engine{command: "not-npm", current_dir: "./app", arg: "start"};
        let status = broken_engine.run_command(Environment::Development);
        assert!(status.is_err());
    } 
}