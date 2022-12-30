use clap::{crate_description, crate_name, crate_version, Arg, ColorChoice, Command};

#[derive(Debug, Default)]
pub struct Settings {
    pub filename: Option<String>,
    pub debug_token: bool,
    pub debug_ast: bool,
    pub debug_graph: bool,
}

pub fn cargs() -> Settings {
    let matches = Command::new(crate_name!())
        .color(ColorChoice::Always)
        .version(crate_version!())
        .author("Cowboy8625")
        .about(crate_description!())
        .arg(Arg::new("filename"))
        .arg(
            Arg::new("debug-token")
                .long("debug-token")
                .required(false)
                .action(clap::ArgAction::SetTrue)
                .help("Show Tokens as they are created"),
        )
        .arg(
            Arg::new("debug-ast")
                .long("debug-ast")
                .required(false)
                .action(clap::ArgAction::SetTrue)
                .help("Show Ast"),
        )
        .arg(
            Arg::new("debug-graph")
                .long("debug-graph")
                .required(false)
                .action(clap::ArgAction::SetTrue)
                .help("Turns AST into a visual graph"),
        )
        .get_matches();

    let mut setting = Settings::default();
    if let Some(filename) = matches.get_one::<String>("filename") {
        setting.filename = Some(filename.to_string());
    }
    setting.debug_token = *matches
        .get_one::<bool>("debug-token")
        .expect("debug-token failed");
    setting.debug_ast = *matches
        .get_one::<bool>("debug-ast")
        .expect("debug-ast failed");
    setting.debug_graph = *matches
        .get_one::<bool>("debug-graph")
        .expect("debug-graph failed");
    setting
}
