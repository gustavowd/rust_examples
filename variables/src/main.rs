fn main() {
    // int
    let idade = 43; // i32 é padrão
    let idade2:u8 = 29;

    // bol
    let teste1:bool = true;
    let teste2 = false;

    // float
    let saldo1 = 9999.99;   // f64 é padrão
    let saldo2:f32 = 1.99;   // evite usar o f32

    // array
    let pgtos_dias = [5, 15 ,20];
    let boletos:[f64; 4] = [459.90, 36.10, 123.00, 9.99];

    println!("Gustavo tem {} anos de idade!", idade);
    println!("Meu aluno tem {} anos de idade!", idade2);

    let mut total:f64 = 0.0;
    for x in boletos.iter(){
        total += *x;
    }
    println!("Total a pagar: {}", total);

    let boletos2 = vec![459.90, 36.10, 123.00, 9.99];
    let mut total2:f64 = 0.0;
    for x in boletos2{
        total2 += x;
    }
    println!("Total a pagar: {}", total2);

    let cdftv = (235000, 10.190, true, 'A');
    let cdftv2: (i32, f64, bool, char) = (235000, 10.190, true, 'A');

    println!("Tuple index 0 is {}", cdftv.0);
    println!("Tuple index 1 is {}", cdftv.1);
    println!("Tuple index 2 is {}", cdftv.2);
    println!("Tuple index 3 is {}", cdftv.3);
    println!("tuple of tuples: {:?}", cdftv2);

    // slice
    let array: [i32; 5] = [0, 1, 2, 3, 4];
    let slice = &array[0..3];
    for x in slice {
        println!("x is {}", x);
    }

    //struct
    struct StTest {
        nome:String,
        inscritos:u32,
        mediaLikes:f64
    };

    let var = StTest {
        nome:String::from("Sistemas Embarcados"),
        inscritos:60,
        mediaLikes: 70.0
    };

    println!("Melhor canal do YT: {}", var.nome);

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let localhost = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // Create an array of 20 elements where all elements are the same.
    // The size should be a compile-time constant.
    let ones = [1; 20];
    // Get the length of an array.
    println!("Length of ones: {}", ones.len());

    // Create a mutable empty vector
    let mut vector = Vec::new();
    vector.push(20);
    vector.push(15);
    vector.insert(0, 10); // insert at the beginning
    println!("First element of vector: {}", vector[0]); // 10
    println!("Second element of vector: {}", vector[1]); // 20
    println!("Third element of vector: {}", vector[2]); // 15
    // Create a vector using the `vec!` macro
    let till_five = vec![1, 2, 3, 4, 5];
    // Create a vector of 20 elements where all elements are the same.
    let ones = vec![1; 20];
    // Get the length of a vector.
    println!("Length of ones: {}", ones.len());
    // Run-time bounds-check.
    // This panics with 'index out of bounds: the len is 5 but the index is 5'.
    //println!("Non existant element of array: {}", till_five[5]);
}
