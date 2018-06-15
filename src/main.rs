mod lexer;

fn main() {

  let input = "
      let five = 5;
      five == 5;
      five != 5;
      five >= 5;
      five <= 5;
    ".to_string();
  let lexy = lexer::Lexer::new(input);
  println!("the lexing tokens 3: {:?}", lexy.tokens);
  println!("Hello, world!");
}


