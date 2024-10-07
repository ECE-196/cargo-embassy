use crate::{cli::init_args::InitArgs, error::Error};
use std::{env::set_current_dir, fs, io::Write, process::Command};

pub struct Init;

impl Init {
    pub fn run(&self, args: InitArgs) {
        if let Err(e) = self.run_inner(args) {
            println!("Failed with error: {e:#?}.");
        } else {
            println!("Finished.");
        }
    }

    fn run_inner(&self, args: InitArgs) -> Result<(), Error> {
        self.create_project(&args.name)?;

        self.init_config()?;
        self.init_toolchain()?;
        self.init_build()?;
        self.init_manifest(&args.name)?;
        self.init_main()?;

        Ok(())
    }

    fn create_project(&self, name: &str) -> Result<(), Error> {
        Command::new("cargo")
            .args(["new", &name])
            .output()
            .map_err(|_| Error::CreateCargo)?;

        set_current_dir(name).map_err(|_| Error::ChangeDir)
    }

    fn init_config(&self) -> Result<(), Error> {
        fs::create_dir_all(".cargo").map_err(|_| Error::CreateFolder(".cargo".into()))?;

        self.create_file(
            ".cargo/config.toml",
            include_str!("templates/config.toml.template"),
        )
    }

    fn init_toolchain(&self) -> Result<(), Error> {
        self.create_file(
            "rust-toolchain.toml",
            include_str!("templates/rust-toolchain.toml.template"),
        )
    }

    fn init_build(&self) -> Result<(), Error> {
        self.create_file("build.rs", include_str!("templates/build.rs.template"))
    }

    fn init_manifest(&self, name: &str) -> Result<(), Error> {
        self.create_file(
            "Cargo.toml",
            &format!(include_str!("templates/Cargo.toml.template"), name = name),
        )
    }

    fn init_main(&self) -> Result<(), Error> {
        self.create_file("src/main.rs", include_str!("templates/main.rs.template"))
    }

    fn create_file(&self, name: &str, content: &str) -> Result<(), Error> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(name)
            .map_err(|_| Error::CreateFile(name.into()))?;

        file.write_all(content.as_bytes())
            .map_err(|_| Error::CreateFile(name.into()))?;

        Ok(())
    }
}
