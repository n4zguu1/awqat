use crate::schema::db_creation;
use std::path::Path;

#[path = "build/parser/mod.rs"]
mod parser;
#[path = "build/schema.rs"]
mod schema;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let path = Path::new("data/data.db");
    db_creation(path).expect("failed to create data base");
}
