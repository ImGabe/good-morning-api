use std::collections::HashSet;
use std::fs;
use std::io;
use std::sync::RwLock;

use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, launch, routes, State};

struct RwLockHashSet(RwLock<HashSet<String>>);

#[derive(Deserialize, Serialize)]
struct Response {
    name: String,
    link: String,
}

#[get("/")]
async fn index(state: &State<RwLockHashSet>) -> Json<Vec<Response>> {
    let state = state.0.read().expect("RwLockHashSet failed");
    let mut vec = Vec::new();

    for image in state.iter() {
        vec.push(Response {
            name: image.to_string(),
            link: format!("http://localhost:8000/images/{}", image),
        })
    }

    Json(vec)
}

#[get("/images/<id>")]
async fn image(id: &str) -> io::Result<NamedFile> {
    NamedFile::open(format!("static/{}.png", id)).await
}

#[launch]
fn rocket() -> _ {
    let files = fs::read_dir("static").expect("Failed to find file");
    let images = HashSet::from(
        files
            .filter_map(Result::ok)
            .map(|f| {
                f.path()
                    .file_stem()
                    .expect("Failed lifetime")
                    .to_owned()
                    .into_string()
                    .expect("Failed to convert to string")
            })
            .collect(),
    );

    rocket::build()
        .manage(RwLockHashSet(RwLock::new(images)))
        .mount("/", routes![index, image])
}
