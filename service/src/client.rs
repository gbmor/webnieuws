//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use crate::db;
use rocket::response::content;

#[get("/")]
pub fn handle() -> content::Json<String> {
    let cache = db::CACHE.read();

    let mut posts = Vec::new();
    (*cache).iter().for_each(|(_, v)| {
        posts.push(v.clone());
    });
    content::Json(str_to_json(posts))
}

fn str_to_json(data: Vec<Vec<String>>) -> String {
    let mut out = String::from("{\n");
    data.iter().enumerate().for_each(|(i, e)| {
        out.push_str(&format!(
            "\n\t\"{}\": {{\n\t\t\"author\": \"{}\",\n\t\t\"body\": \"{}\",\n\t\t\"date\": \"{}\",\n\t\t\"tags\": \"{}\"\n\t}}",
            e[1], e[0], e[2], e[3], e[4]
        ));
        if data.len() > 1 && i < data.len()-1 {
            out.push_str(",\n");
        } else {
            out.push_str("\n");
        }
    });
    out.push_str("}");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_json() {
        let lhs = format!("{{ \"author\": \"test\", \"title\": \"test\", \"body\": \"test\", \"date\": \"test\", \"tags\": \"test\" }}");

        let rhs: Vec<String> = vec![
            "test".into(),
            "test".into(),
            "test".into(),
            "test".into(),
            "test".into(),
        ];
        let rhs = vec![rhs];
        let rhs = str_to_json(rhs);

        assert_eq!(lhs, rhs);
    }
}
