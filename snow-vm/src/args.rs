use clap::{crate_description, crate_name, crate_version, Arg, ColorChoice, Command};

#[derive(Debug, Default)]
pub struct Settings {
    pub filename: Option<String>,
}

pub fn cargs() -> Settings {
    let matches = Command::new(crate_name!())
        .color(ColorChoice::Always)
        .version(crate_version!())
        .author("Cowboy8625")
        .about(crate_description!())
        .arg(Arg::new("filename"))
        .get_matches();

    let mut setting = Settings::default();
    if let Some(filename) = matches.get_one::<String>("filename") {
        setting.filename = Some(filename.to_string());
    }
}
