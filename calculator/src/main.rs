use clap::{App, Arg};

fn main() {
    let app = App::new("calculator")
        .arg(
            Arg::with_name("sum")
                .long("--sum")
                .takes_value(true)
        )

        .get_matches();


    if let Some(numbers) = app.value_of("sum") {
        let result: u32 = numbers
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .sum();
        println!("Result: {}", result.to_string());
    }

/*
    //let teste = "1 2 3 4 5".to_string();
    let teste = "1 2 3 4 5";
    let result2: u32 = teste
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .sum();
    println!("Result2: {}", result2.to_string());
*/

}
