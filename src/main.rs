
use std::collections::HashMap;

#[cfg(test)]
mod lexer {

  #[derive(Debug)]
  #[derive(PartialEq)]
  enum Token {
    //Illegal,
    //Eof,
    Assign,
    Plus,
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,

    ForwardSlash,
    BackwardSlash,
    Equals,
    NotEqual,
    Asterix,
    Bang,

    Comma,
    Semicolon,
    Other(char),
    Let,
    Lambda,
    Identifier(String),
    Number(i32),
    EOF
  }

  #[derive(Debug)]
  #[derive(PartialEq)]
  struct Lexer {
    input: String,
    tokens: Vec<Token>
  }

  impl Lexer {
    fn new(input: String) -> Lexer  {
      //for each char in string, add a token to a struct and assign it to a vector
      let tokens: Vec<Token> = tokenizer(&input);

      return Lexer {
        input: input,
        tokens: tokens
      };
    }
  }

  fn tokenizer (input : &String) -> Vec<Token>{
    let mut tokens: Vec<Token> = vec![];
    let mut chunk : String =  "".to_string();

    for (index, ch) in input.chars().enumerate() {
      // println!("ch: {:?},  chunk: {:?}", ch, chunk);
      if is_whitespace(ch) {
        //flush whatever is in the chunk to the tokenlist
        if !chunk.is_empty() {
          tokens.push(map_keyword(chunk.clone()));
          chunk.clear();
        }
      } else if is_valid_identifier_char(ch) {
        //add it to the chunk
        chunk.push(ch);
      } else {
        // clear the chunk to the list
        if !chunk.is_empty() {
          tokens.push(map_keyword(chunk.clone()));
          chunk.clear();
        }
        // add the char as a token
        tokens.push(map_to_token(ch));
      }
    }

    tokens.push(Token::EOF);

    return tokens;
  }

  fn map_keyword(keyword: String) -> Token {
    // println!("mapping keyword to string: {} ", keyword);
    if is_number(&keyword) {
      let num: i32 = keyword.parse().unwrap(); 
      Token::Number(num)
    } else {
      match &keyword as &str {
        "let" => Token::Let,
        "fn" => Token::Lambda,
        _ => Token::Identifier(keyword.clone())
      }
    }
  }

  fn map_to_token(ch : char) -> Token {
    match ch {
      '=' => Token::Assign,
      '+' => Token::Plus,
      '(' => Token::ParenOpen,
      ')' => Token::ParenClose,
      '{' => Token::BracketOpen,
      '}' => Token::BracketClose,
      ',' => Token::Comma,
      ';' => Token::Semicolon,
      '!' => Token::Bang,
      '/' => Token::ForwardSlash,
      '\\' => Token::BackwardSlash,
      _ => Token::Other(ch)
    }
  }
  
  // returns true if it can be used in a keyword
  fn is_valid_identifier_char(ch : char) -> bool {
    (('a' <= ch) && (ch <= 'z')) || (('A' <= ch) && (ch <= 'Z') || is_digit(ch))
  }

  // is whitespace
  fn is_whitespace(ch : char) -> bool {
    (ch == ' ') || (ch == '\n') || (ch == '\t')
  }

  fn is_digit(ch : char) -> bool {
    (ch >= '0') && (ch <= '9')
  }

  fn is_number(ident : &String) -> bool {
    // its a number if all the chars are numbers
    let mut has_non_number : bool = false;
    for ch in ident.chars() {
      if !is_digit(ch) {
        has_non_number = true;
      }
    }
    !has_non_number
  }


  #[test]
  fn test_next_token_with_basic_stuff() {
    let input = "=+(){},;".to_string();
    let output: Vec<Token> = vec![
      Token::Assign,
      Token::Plus,
      Token::ParenOpen,
      Token::ParenClose,
      Token::BracketOpen,
      Token::BracketClose,
      Token::Comma,
      Token::Semicolon,
      Token::EOF
    ];

    let lexer:Lexer = Lexer::new(input);
    // println!("the lexing tokens: {:?}", lexer);
    for (index, token) in lexer.tokens.iter().enumerate() {
      assert_eq!(token, &output[index]);
    }

    assert_eq!(2, 2);
  }

  #[test]
  fn test_for_keywords() {
    let input = "
      let five = 5;
      let ten = 10;
      let add = fn(x, y) {
          x + y;
      };
      let result = add(five, ten);
    ".to_string();
    let lexer:Lexer = Lexer::new(input);
    println!("the lexing tokens 2: {:?}", lexer.tokens);

    /*
    let test_list: Vec<Token> = vec![
      Let,
      Identifier("five"),
      Assign,
      Number(5),
      Semicolon,
      Let,
      Identifier("ten"),
      Assign,
      Number(10),
      Semicolon, Let, Identifier("add"), Assign, Lambda, ParenOpen, Identifier("x"), Comma, Identifier("y"), ParenClose, BracketOpen, Identifier("x"), Plus, Identifier("y"), Semicolon, BracketClose, Semicolon, Let, Identifier("result"), Assign, Identifier("add"), ParenOpen, Identifier("five"), Comma, Identifier("ten"), ParenClose, Semicolon, EOF];
    */

    assert_eq!(lexer.tokens[0], Token::Let);

    assert_eq!(lexer.tokens[6], Token::Identifier("ten".to_string()));
  }
}


fn main() {
    println!("Hello, world!");
}


