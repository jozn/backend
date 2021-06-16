
use tokio::fs::File;

use tokio_util::codec::{BytesCodec, FramedRead};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Result, Server, StatusCode};

static INDEX: &str = "examples/send_file_index.html";
static NOTFOUND: &[u8] = b"Not Found";



#[tokio::main]
async fn main() {
    // pretty_env_logger::init();

    let addr = "127.0.0.1:1337".parse().unwrap();

    let make_service =
        make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(response_examples)) });

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn response_examples(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => simple_file_send(INDEX).await,
        (&Method::GET, "/no_file.html") => {
            // Test what happens when file cannot be be found
            simple_file_send("this_file_should_not_exist.html").await
        }
        _ => Ok(not_found()),
    }
}

/// HTTP status code 404
fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOTFOUND.into())
        .unwrap()
}

async fn simple_file_send(filename: &str) -> Result<Response<Body>> {
    // Serve a file by asynchronously reading it by chunks using tokio-util crate.

    if let Ok(file) = File::open(filename).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        return Ok(Response::new(body));
    }

    Ok(not_found())
}




fn main2() {
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
}

mod serving {

}

struct Bucket {
    bucket_id: u32,
}

struct File22 {
    bucket_id: u32,
    file_id: u64,
    ref_id: u64,
    secret: u32,

    created_time: u32,
}

mod args {}
mod file_serving {}
mod types {}

struct Config{}
struct FileReq{} // file_id,...
fn sync(){}
fn upload(){}

// in db:
//  file