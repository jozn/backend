extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_qs as qs;


fn main() {
    let c = clap::App::new("my app")
        .version("234")
        .long_version("this is long versino ")
        .long_about("and this kis long about")
        .author("hamid")
        .about("flip filr server")
        .arg("-m 'sert my app'")
        .arg(clap::Arg::new("port")
            .short('p')
            .long_about("long about setiing port goes inot herer")
            .long("port")
            .about("set port this way")
            .default_value("2343")
            .required(false)

        )
        .subcommand(clap::App::new("remove")
            .about("sdfsd")
            .version("v123")
            .arg("-s, --sort 'file Sorting inof'"))
        .get_matches();

    let p = c.value_of("port");
    println!(">> Porting: {:?}", p);

    let f = File{
        bucket_id: 234,
        file_id: 2,
        ref_id: 23432,
        secret: 3,
        created_time: 0
    };

    let s = serde_qs::to_string(&f);
    println!(">> Porting: {:?}", s);


}

mod serving {

}

struct Bucket {
    bucket_id: u32,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct File {
    bucket_id: u32,
    file_id: u64,
    ref_id: u64,
    secret: u32,

    created_time: u32,
}

// in db:
//  file