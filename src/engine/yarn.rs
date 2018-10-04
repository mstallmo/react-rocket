use rocket::config::Environment;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;
use std::process::{Command, Stdio};

#[derive(Default)]
pub struct Yarn {}

impl Fairing for Yarn {
    fn info(&self) -> Info {
        Info {
            name: "Run yarn start on development startup",
            kind: Kind::Launch,
        }
    }

    fn on_launch(&self, rocket: &Rocket) {
        match rocket.config().environment {
            Environment::Development => {
                info!("💨  Ignition sequence start...");
                Command::new("yarn")
                    .current_dir("./app")
                    .arg("start")
                    .stdout(Stdio::null())
                    .spawn()
                    .expect("yarn failed");
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
