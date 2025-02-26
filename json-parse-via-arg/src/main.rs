// JSON PARRSE 2
// Exemplo de desserialização de um arquivo JSON para um vetor de estruturas
// e acesso a um campo opcional de um campo opcional de uma estrutura.



use std::env;
use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;



#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Key3 {
    key4: i32,
    key5: String,
    key6: Option<Vec<f64>>, // key6 é agora opcional
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct MyData {
    key1: String,
    key2: String,
    key3: Key3,
}

fn main() {
    // Obtém os argumentos passados na linha de comando
    let args: Vec<String> = env::args().collect();

    // Verifica se o usuário passou um caminho para o arquivo JSON
    if args.len() < 2 {
        eprintln!("Uso: {} <caminho_do_arquivo_json>", args[0]);
        std::process::exit(1);
    }

    

    let file_path = &args[1]; // Primeiro argumento é o caminho do JSON

    let file:File = File::open(file_path).expect("Erro ao abrir o arquivo");

    let reader = BufReader::new(file);

    // Desserializa o conteúdo do arquivo para um vetor de MyData
    let my_data: Vec<MyData> = serde_json::from_reader(reader).expect("Erro ao desserializar o JSON");

    // Acessando e imprimindo apenas o segundo valor de key6 (índice 1) caso exista
    for data in my_data {
        // Verifica se key6 existe e se tem um índice 1
        if let Some(key6) = &data.key3.key6 {
            if let Some(value) = key6.get(1) {
                println!("Valor de key6[1]: {}", value);
            } else {
                println!("key6 não possui índice 1.");
            }
        } else {
            println!("key6 não existe.");
        }
    }
}
