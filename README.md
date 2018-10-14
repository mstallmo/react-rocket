# Igniter 💨 🔥

[![Build Status](https://travis-ci.com/mstallmo/rocket_igniter.svg?branch=master)](https://travis-ci.com/mstallmo/rocket_igniter)
[![Current Crates.io Version](https://img.shields.io/crates/v/rocket_igniter.svg)](https://crates.io/crates/rocket_igniter)

Igniter is a Rocket.rs fairing for a better frontend development experience. Igniter let's you launch your frontend dev environment when starting your rocket application.

## CLI Support
We currently support NPM and Yarn as CLIs for interaction with your frontend application with hopes to expand to Ember and Vue coming shortly.

## Getting Started

### Usage

Add the crate as a dependency to your project:

```
cargo install rocket_igniter
```

Once the crate is installed you will need to include CliCommand and Engine to configure the desired CLI and start the engine on application start:
```
extern crate rocket_igniter;

use rocket_igniter::engine::{CliCommand, Engine};
```

Create a new Engine by passing it the desired CLI enum:
```
Engine::new(CliCommand::NPM)
```
Attach the create engine to your application as a fairing:
```
fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![hello_world],
        )
        .attach(Engine::new(CliCommand::NPM))
        .launch();
}
```
Start your rocket application and see your development server startup too!

Igniter only runs when in the development environment. If the Rocket environment is configured to be Stage or Production Igniter will get out of your way and happily do nothing!


### Configuration

Igniter has default configurations that come out of the box. By default igniter expects your frontend application code to exist under the `./app` directory. If you want your frontend application code to exist elsewhere you can override this by setting the `igniter_app_dir` key in your Rocket.toml file under the `[development]` heading. For instance if you wanted to place your code under a directory named `myApp` the configuration would look like this: 
```
[development]
igniter_app_dir = "./myApp"
```

On application startup Igniter will run the `start` script in your `package.json`. Along with the code location this script can be overridden by configuration in Rocket.toml. If your application runs it's development environment by running a different script such as `local` instead of `start` your settings would look like this:
```
[development]
igniter_arg = "local"
```
This will run `npm run local` under the hood instead of `npm run start`.

If your application has it's code under `./app` and runs the development server with `start` then you don't have to touch any configuration!

## Issues
As this project is just maintained by me we don't have an offical gitter or discourse. If you run into any issues or have any feature requests please open an issue in the repository! It would be greatly appreciated if you use the issue templates when opening a new issue! Thank you in advance!

## Code of Conduct
Anyone who interacts with Diesel in any space, including but not limited to
this GitHub repository, must follow our [code of conduct](https://github.com/mstallmo/rocket_igniter/blob/master/CODE_OF_CONDUCT.md).


## License
Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

## Contributing
Unless you explicitly state otherwise, any contribution you intentionally submit
for inclusion in the work, as defined in the Apache-2.0 license, shall be
dual-licensed as above, without any additional terms or conditions.
