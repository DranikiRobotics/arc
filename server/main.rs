use robot::pyruntime::PyRuntime;

use arc_rs::html::*;
arc_rs::inc!();

#[cfg(debug_assertions)]
const SAVE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/code");
#[cfg(not(debug_assertions))]
const SAVE_DIR: &str = "/arc/code";

#[cfg(debug_assertions)]
const TEMP_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tmp");
#[cfg(not(debug_assertions))]
const TEMP_DIR: &str = "/arc/tmp";

#[get("/")]
fn index() -> Response {
    res!(@HTML #200 include_hbs!("./html/index.html"))
}

#[get("/")]
fn code() -> Response {
    res!(@HTML #200 include_hbs!("./html/code/index.html"))
}

#[post("/upload?<name>", format = "plain", data = "<input>")]
async fn upload<'a>(name: &'a str, input: rocket::Data<'_>) -> Result {
    use rocket::data::ToByteUnit;
    let location = format!("{SAVE_DIR}/{}", name);
    let file = match rocket::tokio::fs::File::create(location).await {
        Err(e) => return Ok(res!(@JSON #500 format!(
            "{{\"ok\":false,\"msg\":\"Failed to create file: {e}\"}}"
        ))),
        Ok(file) => file,
    };
    if let Err(e) = input.open(1.megabytes()).stream_to(file).await {
        return Ok(res!(@JSON #500 format!(
            "{{\"ok\":false,\"msg\":\"Failed to write to file: {e}\"}}"
        )));
    }
    Ok(res!("{\"ok\":true}".into()))
}

fn main() {
    let mut python_runtime = PyRuntime::init();
    
    python_runtime.start("auto").unwrap();
}

// #[rocket::main]
// async fn main() {
//     let python_runtime = PyRuntime::init();
// 
//     let config = rocket::Config {
//         temp_dir: TEMP_DIR.into(),
//         ident: make_server_identifier(),
//         ..Default::default()
//     };
//     
//     let rocket = rocket::custom(&config)
//         .mount("/", routes![index])
//         .mount("/", routes![assets::style, assets::favicon, assets::font, assets::logo_white_a])
//         .mount("/code", routes![code, upload])
//         .register("/", catchers![error_catcher])
//         .ignite()
//         .await
//         .expect("Failed to start server");
//     rocket.launch()
//         .await
//         .expect("Failed to launch server");
// }