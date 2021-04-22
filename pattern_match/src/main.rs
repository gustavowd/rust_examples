use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    
    let value = match args[1].as_str(){
        "42" => "Resposta para a vida, o universo e tudo mais",
        "22" | "11" => "Pato",
        _ => "Deu ruim!",
    };

    println!("{}", value);
}
