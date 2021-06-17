use crate::types::{Config};

pub fn get_cli_args() -> Config {
    use clap::{App, Arg};

    let c = App::new("Flip Storage Chunk")
        .version("0.1.0")
        .long_version("This is version is not production ready yet")
        .long_about("Chunk servers static files of Flips")
        .author("Hamid Karimi")
        .about("Flip File Storage (FFS) - Chunk Server")
        //.arg("-m 'sert my app'")
        .arg(clap::Arg::new("port")
            .short('p')
            .long("port")
            .about("Set port for serving files")
            .long_about("All listening connection are set with this.")
            .default_value("9000")
            .required(false)
        )
        .arg(Arg::new("dir")
            .short('d')
            .long("dir")
            .about("Path of files Directory (multiple)")
            .default_value("./")
            .required(false)
            .multiple_values(true)
        )
        .get_matches();

    let port = c.value_of("port").unwrap().parse::<u16>().unwrap();

    let dir_vals = c.values_of("dir").unwrap();
    let dirs = dir_vals.into_iter().map(|d| d.to_string()).collect();

    Config {
        port: port,
        dirs: dirs,
        db_path: "".to_string()
    }
}