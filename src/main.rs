use csv::{ReaderBuilder, StringRecord};
use std::collections::{HashMap};
use std::{fs};

const FILENAME: &str = "history.csv";

// TIPO, TAG, TEXTO, VIDA
#[derive(Debug)]
struct HistoryData {
    data_type: String,
    text: String,
    tag: String,
    life: i32,
    options: Vec<HistoryData>
}

impl HistoryData {
    fn new(row: StringRecord) -> HistoryData {
        return HistoryData {
            data_type: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            life: row.get(3).unwrap().trim().parse::<i32>().unwrap_or(0),
            options: Vec::new()
        };
    }
}

fn main() {

    let mut life = 100;
    let mut actual_tag = "INICIO".to_string();
    let mut last_record = "".to_string();

    let content = fs::read_to_string(FILENAME).unwrap();
    let mut history_data: HashMap<String, HistoryData> = HashMap::new();


    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    for result in rdr.records() {
        let record = result.unwrap();
        let data = HistoryData::new(record);

        if data.data_type == "SITUACION" {
            let record_tag = data.tag.clone();
            history_data.insert(record_tag.clone(), data);
            last_record = record_tag;
        } else if data.data_type == "OPCION" {
            if let Some(local_data) = history_data.get_mut(&last_record) {
                (*local_data).options.push(data);
            }
        }

    }

    // Game loop
    loop {
        println!("Tienes {} de vida", life);
        
        if let Some(data) = history_data.get(&actual_tag){
            println!("{}", data.text);

            for (indice, option) in data.options.iter().enumerate() {
                println!("[{}] {}", indice, option.text);
            }

            let mut seleccion = String::new();
            std::io::stdin().read_line(&mut seleccion).unwrap();
            let seleccion = seleccion.trim().parse().unwrap_or(99);

            if let Some(opcion_elegida) = &data.options.get(seleccion){
                actual_tag = opcion_elegida.tag.clone();
            }else{
                println!("Comando no valido");
            }


            life += data.life;
            println!("");
        }else{
            break;
        }

        // Si la vida <= 0 entonces terminar juego
        if life <= 0{
            println!("Has perdido!");
            break;
        }

    }
}
