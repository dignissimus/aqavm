use aqavm::lexer;

fn main() {
    println!("{:?}",
             lexer::tokenize(
                 r#"OUTPUT("Hello, World!")
                 INPUT(0x22)"#
                     .to_string()
             )
    );
}
