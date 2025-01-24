use csv::{ReaderBuilder, StringRecord};
use std::{fs};

const FILENAME: &str = "history.csv";

// TIPO, TAG, TEXTO, VIDA
#[derive(Debug)]
struct HistoryData {
    data_type: String,
    text: String,
    tag: String,
    life: i32
}


fn main() {

    let content = fs::read_to_string(FILENAME).unwrap();
    let mut history_data: Vec<HistoryData> = Vec::new();

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    for result in rdr.records() {
        let record = result.unwrap();
        let history_recod = HistoryData {
            data_type: record.get(0).unwrap().trim().to_string(),
            tag: record.get(1).unwrap().trim().to_string(),
            text: record.get(2).unwrap().trim().to_string(),
            life: record.get(3).unwrap().trim().parse::<i32>().unwrap_or(0)
        };
        history_data.push(history_recod);
        //println!("Texto: {}", result.unwrap().get(2).unwrap());
    }

    println!("{:?}", history_data);
}
