use clap::{Arg, Command};

mod instagram;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("compressta")
        .about("Share your Instagram photos as CNFT's")
        .subcommand(
            Command::new("create_tree")
                .about("Create a new Bubblegum Tree")
                .arg(
                    Arg::new("Max depth")
                        .short('d')
                        .long("max-depth")
                        .required(true),
                )
                .arg(
                    Arg::new("Max buffer size")
                        .short('b')
                        .long("max-buffer-size")
                        .required(true),
                )
                .arg(Arg::new("Public").short('p').long("public").required(false)),
        )
        .get_matches();

    Ok(())
}
