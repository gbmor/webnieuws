//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use std::io::Write;
use std::net::TcpStream;

use crate::db;

use rusqlite;

pub fn handle(strm: &mut TcpStream) -> rusqlite::Result<()> {
    let cache = db::CACHE.read().unwrap();

    let mut posts = Vec::new();

    (*cache).iter().for_each(|(_, v)| {
        posts.push(v.clone());
    });

    let posts = str_to_json(posts).bytes().collect::<Vec<u8>>();

    let posts_displayed = if posts.len() < 10 {
        &posts
    } else {
        &posts[posts.len() - 10..]
    };

    strm.write_all(&posts_displayed).unwrap();

    Ok(())
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
