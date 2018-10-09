use rocket::config::Environment;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;
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

    pub fn run_command(&self, environment: Environment) {
        match environment {
            Environment::Development => {
                info!("💨  Ignition sequence start...");
                Command::new(self.command)
                    .current_dir(self.current_dir)
                    .arg(self.arg)
                    .spawn()
                    .expect(&format!("{} failed", self.command));
                info!("🔥  All engines running!");
            }
            _ => (),
        };
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
        self.run_command(rocket.config().environment);
    }
}
