//  JSON Parse 1
//  Realziar a leitura de um arquivo JSON e desserializar o conteúdo em um HashMap
//  O arquivo JSON deve conter um array de objetos
//  O HashMap deve ter como chave o campo key1 do objeto e como valor o objeto completo
//  O HashMap deve ser impresso na tela
//  O arquivo JSON deve ser lido via caminho absoluto e relativo



use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct MyData {
    key1: String,
    key2: i32,
}


fn main() -> std::io::Result<()> {
    
    println!("====>  JSON Parse  <<====");
    
    // Via Caminho Absoluto
    //let file:File = File::open("c:/lab-rust/json-parse/target/debug/data.json")?;

    //Via Caminho Relativo
    let file:File = File::open("src/data.json")?;

    let reader = BufReader::new(file);

    let hashmap: HashMap<String, MyData> = serde_json::from_reader(reader).expect("Erro as desserializar");

    for (key, value) in &hashmap {

        println!("Chave: {}, Valor: {:?}", key, value);
        
    }

    Ok(())
}
