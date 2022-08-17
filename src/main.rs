use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)] // you can discard this if you want.
struct Value {
   /// First number to add
   #[clap(short = 'o', long = "one")]
   one: i32,

}

fn main() {
   let value = Value::parse();

   let first_value = value.one;

   let one1 = first_value - 32;
   let two2 = one1 * 5/9;
   
   println!("Temp in fahrenheit {}", first_value);
   println!("Temp in celsius {}", two2);
}