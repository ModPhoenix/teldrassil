use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use uuid::Uuid;

use crate::data::node::{model::NewNode, query::new_node};

use super::{DataError, Database};

pub async fn import_wiki_links(db: &Database) -> Result<(), DataError> {
    let file = File::open(Path::new("links.tsv")).expect("file not found");
    println!("Importing from file: {:?}", file);

    let buf_reader = BufReader::new(file);

    let records = read(buf_reader).unwrap();
    println!("Imported {} records", records.len());

    let mut context: HashMap<String, Uuid> = HashMap::new();

    for record in records {
        println!("{} -> {}", record.from, record.to);

        if !context.contains_key(&record.from) {
            let parent_id = context.get(&record.to);

            let node = new_node(
                db,
                NewNode {
                    id: None,
                    name: record.from.clone(),
                    content: "No content".to_string(),
                    parent_id: Some(parent_id.cloned().unwrap_or(Uuid::nil())),
                },
            )
            .await?;

            context.insert(record.from, node.id.uuid());
        } else {
            println!("Already exists");
        }
    }

    Ok(())
}

struct Links {
    pub from: String,
    pub to: String,
}

fn read(buf_reader: BufReader<File>) -> io::Result<Vec<Links>> {
    return Ok(buf_reader
        .lines()
        .flat_map(|line| {
            if let Ok(line) = line {
                let mut pair = line.split("\t");

                let from = pair.next();
                let to = pair.next();

                if let (Some(from), Some(to)) = (from, to) {
                    return Some(Links {
                        from: from.to_string(),
                        to: to.to_string(),
                    });
                }
            }

            None
        })
        .collect::<Vec<Links>>());
}
