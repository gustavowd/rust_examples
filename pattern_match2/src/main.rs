use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    
    if let Ok(x) = &args[1].parse::<i32>(){
        match x {
            42 => println!("Resposta para a vida, o universo e tudo mais"),
            22 => println!("Pato"),
            _ => println!("Opção desconhecida!")
        }
    } else{
        println!("Deu ruim, tente de novo .. digite um número!");
    }

}