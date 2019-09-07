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
    str_to_json(posts);
    content::Json(format!("{{ \"kind\":\"test\" }}"))
}

fn str_to_json(data: Vec<Vec<String>>) -> String {
    data.iter().map(|e| {
        format!(
            "{{ \"author\": \"{}\", \"title\": \"{}\", \"body\": \"{}\", \"date\": \"{}\", \"tags\": \"{}\" }}",
            e[0], e[1], e[2], e[3], e[4]
        )
    }).collect::<Vec<String>>().join("").into()
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
