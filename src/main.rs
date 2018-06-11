


#[cfg(test)]
mod lexer {

  #[derive(Debug)]
  enum Token {
    //Illegal,
    //Eof,
    Equal,
    Plus,
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,
    Comma,
    Semicolon,
    Other(char)
  }

  #[derive(Debug)]
  struct Lexer {
    input: String,
    tokens: Vec<Token>
  }

  impl Lexer {
    fn new(input: String) -> Lexer  {
      //for each char in string, add a token to a struct and assign it to a vector
      let mut tokens: Vec<Token> = vec![];
    
      for c in input.chars() {
        let token = map_to_token(c);
        tokens.push(token)
      }

      return Lexer {
        input: input,
        tokens: tokens
      };
    }

  }

  fn map_to_token(ch : char) -> Token {
    match ch {
      '=' => Token::Equal,
      '+' => Token::Plus,
      '(' => Token::ParenOpen,
      ')' => Token::ParenClose,
      '{' => Token::BracketOpen,
      '}' => Token::BracketClose,
      ',' => Token::Comma,
      ';' => Token::Semicolon,
      _ => Token::Other(ch)
    }
  }


  #[test]
  fn test_next_token() {
    let input = "=+(){},;".to_string();
    let output: Vec<Token> = vec![
      Token::Equal,
      Token::Plus,
      Token::ParenOpen,
      Token::ParenClose,
      Token::BracketOpen,
      Token::BracketClose,
      Token::Comma,
      Token::Semicolon
    ];

    let lexer:Lexer = Lexer::new(input);
    println!("the lexer is {:?}", lexer);


    assert_eq!(2, 2);

  }
}


fn main() {
    println!("Hello, world!");
}


