
fn raindrops(n: u32) -> String {
    let mut raindrop = String::new();

    let mut if_is_factor_push = |factor, sound| {
        if n % factor == 0 {
            raindrop.push_str(sound);
        }
    };

    if_is_factor_push(3, "Pling");
    if_is_factor_push(5, "Plang");
    if_is_factor_push(7, "Plong");

    if raindrop.is_empty(){
        raindrop = n.to_string();
    }

    raindrop
}

fn raindrops2(n: u32) -> String {

    let drops = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    
    let mut raindrop = drops        
        .iter()                                     // itera sobre os itens de drops
        .filter(|(factor, _)| n % factor == 0)      // função closure. Para cada dupla faz o teste de divisivel por factor. Se for, mantém o valor
        .map(|&(_, sound)| sound)                   // função closure. Tira de dentro da tupla o sound. função que recebe sound como empréstimo (referencia) e devolve a string como resultado
        .collect::<String>();                       // Coleta as strings em uma única string (precisa especificar o tipo)

    if raindrop.is_empty(){
        raindrop = n.to_string();
    }

    raindrop
}


fn raindrops3(n: u32) -> String {

    let drops = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    
    let mut raindrop = drops        
        .iter()                                     // itera sobre os itens de drops
        .filter(|(factor, _)| n % factor == 0)      // função closure. Para cada dupla faz o teste de divisivel por factor. Se for, mantém o valor
        .map(|(_, sound)| sound.to_string())                   // função closure. Tira de dentro da tupla o sound. função que recebe sound como empréstimo (referencia) e devolve a string como resultado
        .collect::<String>();

    if raindrop.is_empty(){
        raindrop = n.to_string();
    }

    raindrop
}

static TEST_DATA :[(i32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

fn revolve_string((_, sound): &(i32, &str)) -> String{
    sound.to_string()
}

fn revolve_string2((_, sound): (i32, &str)) -> String{
    sound.to_string()
}

fn main() {
    println!("Raindrops de 30: {}", raindrops(30));
    println!("Raindrops de 25: {}", raindrops2(25));
    println!("Raindrops de 21: {}", raindrops3(21));
    println!("Retira segundo som {}", revolve_string(&TEST_DATA[1]));
    println!("Retira terceiro som {}", revolve_string2(TEST_DATA[2]));
}
