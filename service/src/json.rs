//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use crate::db;
use crate::user;

pub fn from_str(data: Vec<Vec<String>>) -> String {
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

pub fn to_comm(json: serde_json::Value) -> user::Comm {
    let kind = json["kind"]
        .as_str()
        .unwrap()
        .chars()
        .map(|c| c.to_lowercase().to_string())
        .collect::<String>();
    let id = if let Some(val) = json["cont"]["id"].as_u64() {
        val
    } else {
        0
    };
    match &kind[..] {
        "post" => user::Comm::Post(db::Entry {
            id,
            author: json["cont"]["author"].as_str().unwrap().to_string(),
            title: json["cont"]["title"].as_str().unwrap().to_string(),
            body: json["cont"]["body"].as_str().unwrap().to_string(),
            date: json["cont"]["date"].as_str().unwrap().to_string(),
            tags: json["cont"]["tags"]
                .as_str()
                .unwrap()
                .split(" ")
                .map(|c| c.to_string())
                .collect::<Vec<String>>(),
        }),
        "delete" => user::Comm::Delete(id),
        "update" => user::Comm::Update(db::Entry {
            id,
            author: json["cont"]["author"].as_str().unwrap().to_string(),
            title: json["cont"]["title"].as_str().unwrap().to_string(),
            body: json["cont"]["body"].as_str().unwrap().to_string(),
            date: json["cont"]["date"].as_str().unwrap().to_string(),
            tags: json["cont"]["tags"]
                .as_str()
                .unwrap()
                .split(" ")
                .map(|c| c.to_string())
                .collect::<Vec<String>>(),
        }),
        _ => user::Comm::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test::Bencher;

    #[ignore]
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
        let rhs = from_str(rhs);

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_json_to_comm() {
        let rhs = user::Comm::Post(db::Entry {
            id: 0,
            author: "foo barrington".into(),
            title: "foos guide to benchmarks".into(),
            body: "do benchmarks k".into(),
            date: "foo oclock".into(),
            tags: vec!["benchmarks".into(), "testing".into(), "stuff".into()],
        });
        let json = r#"{
                "kind": "post",
                "cont": {
                    "id": 0,
                    "author": "foo barrington",
                    "title": "foos guide to benchmarks",
                    "body": "do benchmarks k",
                    "date": "foo oclock",
                    "tags": "benchmarks testing stuff"
                }
            }"#;

        let json = serde_json::from_str(json);
        let json = json.unwrap();
        let lhs = to_comm(json);

        assert_eq!(lhs, rhs);
    }

    #[bench]
    fn bench_json_to_comm(b: &mut Bencher) {
        b.iter(|| {
            let json = r#"{
                "kind": "post",
                "cont": {
                    "id": 0,
                    "author": "foo barrington",
                    "title": "foos guide to benchmarks",
                    "body": "do benchmarks k",
                    "date": "foo oclock",
                    "tags": "benchmarks testing stuff"
                }
            }"#;
            let json = serde_json::from_str(json).unwrap();
            to_comm(json);
        });
    }
}
