use std::collections::HashSet;

fn is_legal_token (c: &char) -> bool {
  let legal_group_tokens = HashSet::from(['(', ')']);

  legal_group_tokens.contains(&c)
}

pub enum TokenErrors {
  IllegalToken(Vec<char>),
}


pub enum Operator {
  Add,
  Subtract,
  Divide,
  Multiply
}

pub struct Token {
  is_group: bool,
  operation: Operator,
  left: f64,
  right: f64,
}

pub fn tokenize (input: String) -> Result<(), TokenErrors> {
  let mut illegal_tokens: Vec<char> = Vec::new();

  for c in input.trim().chars() {
    if is_legal_token(&c) == true {
    } else {
      illegal_tokens.push(c);
    }
  }

  if illegal_tokens.is_empty() { Ok(()) }
  else { Err(TokenErrors::IllegalToken(illegal_tokens)) }
}

#[cfg(test)]
mod test {
  #[test]
  fn hello () {
      assert_eq!(0, 0);
  }
}