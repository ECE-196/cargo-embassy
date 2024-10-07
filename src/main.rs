pub mod cli;
pub mod error;
mod init;

use clap::Parser;
use cli::{Cargo, Embassy};
use init::Init;

fn main() {
    let Cargo::Embassy(embassy) = Cargo::parse();

    match embassy {
        Embassy::Init(args) => {
            let init = Init;
            init.run(args);
        }
    }
}
