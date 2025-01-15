use clap::{Arg, Command};

mod unpack;
mod pack;


fn main() {      
    // Create the Clap command line app
    let matches = Command::new("SurrealDB backup system")
        .version("0.1.1")
        .author("Maxwell Flitton maxwellflitton@gmail.com")
        .about("Unpacks Docker images into a specified directory")
        .arg(
            Arg::new("command")
                .help("The command to execute (e.g., pack, unpack)")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .value_name("DIRECTORY")
                .help("The directory to do the action on")
        )
        .arg(
            Arg::new("target")
                .short('t')
                .long("target")
                .value_name("TARGET")
                .help("The target directory for the action")
        )
        .get_matches();

    let command = matches.get_one::<String>("command").expect("Command argument is required");

    // Match the command to perform the corresponding action
    match command.as_str() {
        "pack" => {
            let directory = match matches.get_one::<String>("directory") {
                Some(directory) => directory,
                None => { 
                    eprintln!("Directory argument is required for pack to point to the DB data");
                    return;
                }
            };
            let target = match matches.get_one::<String>("target") {
                Some(target) => target,
                None => {
                    eprintln!("Target argument is required for pack to point to where the SST file will be saved");
                    return;
                }
            };
            match pack::export_to_sst(directory, target) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error packing SST: {}", e);
                }
            }
        }
        "unpack" => {
            let directory = match matches.get_one::<String>("directory") {
                Some(directory) => directory,
                None => { 
                    eprintln!("Directory argument is required for unpack");
                    return;
                }
            };
            let target = match matches.get_one::<String>("target") {
                Some(target) => target,
                None => {
                    eprintln!("Target argument is required for unpack");
                    return;
                }
            };
            match unpack::create_rocksdb_with_sst(directory, target) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error unpacking SST: {}", e);
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}