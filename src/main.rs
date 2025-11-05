mod lexer;
mod tokens;

fn main() {
    let text = " module assign x = 8'b10101010;\n                assign y = 3'hA;\n                assign z = 5'd17; endmodule";
    let tokens = lexer::lex(text);
    for token in tokens {
        println!("{:?}", token);
    }
}