//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use crate::db;
use crate::json;

use rocket::response::content;

#[get("/")]
pub fn handle() -> content::Json<String> {
    let cache = db::CACHE.read();

    let mut posts = Vec::new();
    (*cache).iter().for_each(|(_, v)| {
        posts.push(v.clone());
    });
    content::Json(json::from_str(posts))
}
