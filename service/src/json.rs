//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use crate::db;
use crate::user;

pub fn to_comm(json: serde_json::Value) -> user::Comm {
    let kind = json["kind"]
        .as_str()
        .unwrap()
        .chars()
        .map(|c| c.to_lowercase().to_string())
        .collect::<String>();
    let id: u32 = json["cont"]["id"].as_str().unwrap().parse().unwrap();
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
                .split("\t")
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
                .split("\t")
                .map(|c| c.to_string())
                .collect::<Vec<String>>(),
        }),
        _ => user::Comm::None,
    }
}
