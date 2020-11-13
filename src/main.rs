mod files;

use warp::Filter;
use std::net::SocketAddr;
use files::file;

#[tokio::main]
async fn main() {
    let base_folder = "public_http";

    let files = file(base_folder.to_string());
    for file in files.iter() {
        println!("File {{\n    \"name\":\"{}\"\n    \"path\":\"{}\"\n}}",file.name(),file.path());
    }

    pretty_env_logger::init();
    let _readme = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./README.md"));

    let _host = warp::header::<SocketAddr>("host");

    // GET / => README.md
    // GET /ex/... => ./examples/..
    let routes = warp::fs::dir(base_folder).or();

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}