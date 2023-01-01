use clap::{crate_description, crate_name, crate_version, Arg, ColorChoice, Command};

#[derive(Debug, Default)]
pub struct Settings {
    pub filename: Option<String>,
    pub bin_file: bool,
    pub debug: bool,
}

pub fn cargs() -> Settings {
    let matches = Command::new(crate_name!())
        .color(ColorChoice::Always)
        .version(crate_version!())
        .author("Cowboy8625")
        .about(crate_description!())
        .arg(Arg::new("filename"))
        .arg(
            Arg::new("debug")
                .long("debug")
                .short('d')
                .required(false)
                .action(clap::ArgAction::SetTrue)
                .help("Debug info output"),
        )
        .arg(
            Arg::new("bin")
                .long("bin")
                .short('b')
                .required(false)
                .action(clap::ArgAction::SetTrue)
                .help("input file is a binary file"),
        )
        .get_matches();

    let mut settings = Settings::default();
    if let Some(filename) = matches.get_one::<String>("filename") {
        settings.filename = Some(filename.to_string());
    }
    settings.debug = *matches.get_one::<bool>("debug").expect("debug failed");
    settings.bin_file = *matches.get_one::<bool>("bin").expect("bin failed");
    settings
}
