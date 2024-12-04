#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Token {
    Mul,
    Lparen,
    Rparen,
    Lnum(u64),
    Rnum(u64),
    Comma,
}

// Mul Lparen Rparen Lnum Comma Rnum Rparen
// => ParsedMulInstruction

#[derive(Debug)]
pub struct MulInstruction(pub u64, pub u64);

#[derive(Debug, Clone)]
pub struct Parser {
    source: String,
}

impl Parser {
    pub fn new(puzzle: &str) -> Self {
        let src = std::fs::read_to_string(puzzle).unwrap();
        Self { source: src }
    }

    pub fn parse_with_flags(&self) -> Vec<MulInstruction> {
        // skip along until you hit an M
        // attempt to parse Mul instruction with expected seq
        let mut iter = self.source.chars().peekable();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parsed_exprs: Vec<MulInstruction> = Vec::new();
        let mut enabled: bool = true;
        while let Some(ch) = iter.next() {
            println!("Toks: {:?}", tokens);
            println!("Char: {}", ch);
            match ch {
                // Mul token
                'm' => {
                    // peek twice to check for u and l
                    if let Some('u') = iter.peek() {
                        iter.next();
                        if let Some('l') = iter.peek() {
                            iter.next();
                            tokens.push(Token::Mul)
                        }
                    }
                }
                'd' => {
                    if let Some('o') = iter.peek() {
                        iter.next();
                        match iter.peek() {
                            Some('(') => {
                                iter.next();
                                if let Some(')') = iter.peek() {
                                    iter.next();
                                    enabled = true;
                                    println!("**ENABLED**");
                                }
                            }
                            Some('n') => {
                                iter.next();
                                if let Some('\'') = iter.peek() {
                                    iter.next();
                                    if let Some('t') = iter.peek() {
                                        iter.next();
                                        if let Some('(') = iter.peek() {
                                            iter.next();
                                            if let Some(')') = iter.peek() {
                                                iter.next();
                                                enabled = false;
                                                println!("**DISABLED**");
                                            }
                                        }
                                    }
                                }
                            }
                            _ => continue,
                        }
                    }
                }
                num @ '0'..='9' => {
                    let mut str = num.to_string();
                    if let Some(num @ '0'..='9') = iter.peek() {
                        str.push(*num);
                        iter.next();
                        if let Some(num @ '0'..='9') = iter.peek() {
                            str.push(*num);
                            iter.next();
                        }
                    }

                    if let Some(t) = tokens.last() {
                        match t {
                            Token::Lparen => tokens.push(Token::Lnum(str.parse().unwrap())),
                            Token::Comma => tokens.push(Token::Rnum(str.parse().unwrap())),
                            _ => tokens.clear(),
                        }
                    }
                }
                ',' => {
                    if let Some(t) = tokens.last() {
                        match t {
                            Token::Lnum(_) => tokens.push(Token::Comma),
                            _ => {
                                tokens.clear();
                            }
                        }
                    }
                }
                '(' => {
                    if let Some(t) = tokens.last() {
                        match t {
                            Token::Mul => tokens.push(Token::Lparen),
                            _ => {
                                tokens.clear();
                            }
                        }
                    }
                }
                ')' => {
                    if let Some(t) = tokens.last() {
                        if let Token::Rnum(_) = t {
                            tokens.push(Token::Rparen);

                            if let [Token::Mul, Token::Lparen, Token::Lnum(l), Token::Comma, Token::Rnum(r), Token::Rparen, ref rem @ ..] =
                                tokens[..]
                            {
                                if enabled {
                                    parsed_exprs.push(MulInstruction(l, r));
                                    tokens = rem.to_vec();
                                } else {
                                    tokens.clear()
                                }
                            } else {
                                tokens.clear();
                            }
                        }
                    }
                }
                _ => {
                    tokens.clear();
                }
            }
        }

        parsed_exprs
    }

    pub fn parse(&self) -> Vec<MulInstruction> {
        // skip along until you hit an M
        // attempt to parse Mul instruction with expected seq
        let mut iter = self.source.chars().peekable();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parsed_exprs: Vec<MulInstruction> = Vec::new();
        while let Some(ch) = iter.next() {
            // println!("Toks: {:?}", tokens);
            // println!("Char: {}", ch);
            match ch {
                // Mul token
                'm' => {
                    // peek twice to check for u and l
                    if let Some('u') = iter.peek() {
                        iter.next();
                        if let Some('l') = iter.peek() {
                            iter.next();
                            tokens.push(Token::Mul)
                        }
                    }
                }
                num @ '0'..='9' => {
                    let mut str = num.to_string();
                    if let Some(num @ '0'..='9') = iter.peek() {
                        str.push(*num);
                        iter.next();
                        if let Some(num @ '0'..='9') = iter.peek() {
                            str.push(*num);
                            iter.next();
                        }
                    }

                    if let Some(t) = tokens.last() {
                        match t {
                            Token::Lparen => tokens.push(Token::Lnum(str.parse().unwrap())),
                            Token::Comma => tokens.push(Token::Rnum(str.parse().unwrap())),
                            _ => tokens.clear(),
                        }
                    }
                }
                ',' => {
                    if let Some(t) = tokens.last() {
                        match t {
                            Token::Lnum(_) => tokens.push(Token::Comma),
                            _ => {
                                tokens.clear();
                            }
                        }
                    }
                }
                '(' => {
                    if let Some(t) = tokens.last() {
                        match t {
                            Token::Mul => tokens.push(Token::Lparen),
                            _ => {
                                tokens.clear();
                            }
                        }
                    }
                }
                ')' => {
                    if let Some(t) = tokens.last() {
                        if let Token::Rnum(_) = t {
                            tokens.push(Token::Rparen);
                            if let [Token::Mul, Token::Lparen, Token::Lnum(l), Token::Comma, Token::Rnum(r), Token::Rparen, ref rem @ ..] =
                                tokens[..]
                            {
                                parsed_exprs.push(MulInstruction(l, r));
                                tokens = rem.to_vec();
                            } else {
                                tokens.clear();
                            }
                        }
                    }
                }
                _ => {
                    tokens.clear();
                }
            }
        }

        parsed_exprs
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_input() {
        let parser = Parser::new("../../puzzle_input/day3-test.txt");
        let exprs = parser.parse();
        println!("Expressions: {:?}", exprs);
        let res = exprs.iter().fold(0u64, |mut acc, x| {
            acc += x.0 * x.1;
            acc
        });
        assert_eq!(res, 161);
    }

    #[test]
    fn real_input() {
        let parser = Parser::new("../../puzzle_input/day3.txt");
        let exprs = parser.parse();
        println!("Expressions: {:?}", exprs);
        let res = exprs.iter().fold(0u64, |mut acc, x| {
            acc += x.0 * x.1;
            acc
        });
        assert_eq!(res, 173419328);
    }

    #[test]
    fn test_input_pt2() {
        let parser = Parser::new("../../puzzle_input/day3-test.txt");
        let exprs = parser.parse_with_flags();
        println!("Expressions: {:?}", exprs);
        let res = exprs.iter().fold(0u64, |mut acc, x| {
            acc += x.0 * x.1;
            acc
        });
        assert_eq!(res, 48);
    }

    #[test]
    fn real_input_pt2() {
        let parser = Parser::new("../../puzzle_input/day3.txt");
        let exprs = parser.parse_with_flags();
        println!("Expressions: {:?}", exprs);
        let res = exprs.iter().fold(0u64, |mut acc, x| {
            acc += x.0 * x.1;
            acc
        });
        assert_eq!(res, 90669332);
    }
}
