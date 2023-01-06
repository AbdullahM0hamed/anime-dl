use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("anime")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommands(vec![SubCommand::with_name("dl")
            .about("Download anime")
            .arg(
                Arg::with_name("query")
                    .help("the name of the anime")
                    .required(true)
                    .index(1),
            )])
}
