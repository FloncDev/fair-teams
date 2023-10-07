use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

#[get("/static/<filename..>")]
pub async fn get_file(filename: PathBuf) -> NamedFile {
    NamedFile::open(
        Path::new("./templates")
            .join(filename)
    )
    .await
    .unwrap()
}