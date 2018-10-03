#![feature(plugin, decl_macro, proc_macro_non_items)]

extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request};

#[derive(Default)]
pub struct Printer{}

impl Fairing for Printer {
    fn info(&self) -> Info {
        Info {
            name: "Print hello on request",
            kind: Kind::Request
        }
    }

    fn on_request(&self, _: &mut Request, _: &Data) {
        println!("Hello from the fairing crate");
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
