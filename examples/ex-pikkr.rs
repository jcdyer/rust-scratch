extern crate pikkr;

fn main() {
    let queries = vec![
        "$.f1".as_bytes(),
        "$.f2.f1".as_bytes(),
    ];
    let train_num = 2;

    let mut p = match pikkr::Pikkr::new(&queries, train_num).expect("pikkr");

    let records = vec![
        r#"{"f1": "a", "f2": {"f1": 1, "f2": true}}"#,
        r#"{"f1": "b", "f2": {"f1": 2, "f2": true}}"#,
        r#"{"f1": "c", "f2": {"f1": 3, "f2": true}}"#,
        r#"{"f2": {"f1": 4, "f2": true}, "f1": "d"}"#,
        r#"{"f2": {"f2": true, "f1": 5}}"#,
        r#"{"f1": "e"}"#,
        r#"{"f3": ["wut", 1]}"#,
    ];

    for record in records {
        match p.parse(record.as_bytes()) {
            Ok(results) => {
                for result in results {
                    print!("{} ", match result {
                        Some(result) => String::from_utf8(result.to_vec()).unwrap(),
                        None => String::from("None"),
                    });
                }
            }
        }
        print!("\n");
    }
}
