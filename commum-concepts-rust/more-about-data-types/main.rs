fn main() {
    type_annotations();
    numeric_operations();
    characters();
    string();
    tuples();
}

fn type_annotations() {
    println!("\ntype_annotations()");

    let number: u32 = "42".parse().expect("Not a number!");
    //let number2 = "42".parse().expect("Not a number!");
    println!("{}", number);
}

/*
error[E0282]: type annotations needed
 --> main.rs:3:9
  |
3 |     let number2 = "42".parse().expect("Not a number!");
  |         ^^^^^^^ consider giving `number2` a type

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
*/

// --------------------------------------------------------------------

/*
Números
Os inteiros no Rust podem ser identificados pelo tamanho de bits e pela propriedade assinada. 
Os inteiros com sinal podem representar números positivos e negativos. 
Os inteiros sem sinal podem representar somente números positivos.

NÚMEROS
Comprimento	Com sinal	Não assinado
8 bits	        i8	        u8
16 bits	        i16	        u16
32 bits	        i32	        u32
64 bits	        i64	        u64
128 bits	    i128	    u128
arco	        isize	    usize

Os tipos isize e usize dependem do tipo de computador em que o seu programa está sendo executado: 64 bits 
se você estiver em uma arquitetura de 64 bits e 32 bits se estiver em uma arquitetura de 32 bits.

Float 
let x = 2.0;      // f64, default type
let y: f32 = 3.0; // f32, via type annotation
*/

fn numeric_operations() {
    println!("\nnumeric_operations()");

    // Addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // ^ Try changing `1i32` to `1u32` to see why the type is important

    // Integer Division
    println!("9 / 2 = {}", 9u32 / 2);

    // Float Division
    println!("9 / 2 = {}", 9.0 / 2.0);

    // Multiplication
    println!("3 * 6 = {}", 3 * 6)
}

fn characters() {
    println!("\ncharacters()");

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{} {} {}", c, z, heart_eyed_cat);
}

fn string() {
    println!("\nstring()");
    let mut hello = String::from("Hello, ");  // create a String from a string literal
    hello.push('w');                          // push a character into our String
    hello.push_str("orld!");                  // push a string literal into our String
    println!("{}", hello)
}

fn tuples() {
    println!("\ntuples()");
    let tuple = ("hello", 5, 'c');

    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');
}
