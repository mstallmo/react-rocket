use rocket::config::Environment;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;
use std::process::{Command, Stdio};

#[derive(Default)]
pub struct NPM;

impl Fairing for NPM {
    fn info(&self) -> Info {
        Info {
            name: "Run npm start on development startup",
            kind: Kind::Launch,
        }
    }

    fn on_launch(&self, rocket: &Rocket) {
        match rocket.config().environment {
            Environment::Development => {
                info!("💨  Ignition sequence start...");
                Command::new("npm")
                    .current_dir("./app")
                    .arg("start")
                    .stdout(Stdio::null())
                    .spawn()
                    .expect("npm failed");
                info!("🔥  All engines running!");
            }
            _ => (),
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}