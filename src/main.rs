// mod lexer;
// mod tokens;

// fn main() {
//     let text = " module assign x == 8'b10101010;\n      [10:9]          assign y <= 3'hA;\n                assign z = 5'd17; endmodule";
//     let tokens = lexer::lex(text);
//     for token in tokens {
//         println!("{:?}", token);
//     }
// }


mod lexer;
mod tokens;
use std::fs;
#[allow(dead_code)]
fn main() {
    // Define the path to the file
    let file_path = "src/test.v";

    // Read the file content into a String
    let text = fs::read_to_string(file_path)
        .expect("Should have been able to read the file. Ensure 'test.v' exists in the src directory.");

    // Perform lexing on the file content
    let tokens = lexer::lex(&text);

    // Print the results
    for token in tokens {
        println!("{:?}", token);
    }
}