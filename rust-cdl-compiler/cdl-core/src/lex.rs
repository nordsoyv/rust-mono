use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum LexItem {
    Identifier(String),
    String(String),
    Reference(String),
    Number { value: f64, real_text: String },
    Colon,
    Comma,
    Equal,
    LessThan,
    MoreThan,
    Percent,
    OpenBracket,
    CloseBracket,
    OpenPar,
    ClosePar,
    Plus,
    Minus,
    Div,
    Mul,
    EOL,
}

pub struct Lexer {
    input: String
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
        }
    }

    pub fn lex(&self) -> Result<Vec<LexItem>, String> {
        let mut result = Vec::new();
        let mut it = self.input.chars().peekable();
        while let Some(&c) = it.peek() {
            match c {
                'a'...'z' | 'A'...'Z' | '_' => {
                    it.next();
                    let ident = get_identifier(c, &mut it);
                    result.push(LexItem::Identifier(ident));
                }
                '0'...'9' => {
                    it.next();
                    let (number, real_text) = get_number(c, &mut it);
                    result.push(LexItem::Number { value: number, real_text });
                }
                '@' => {
                    it.next();
                    let reference = get_reference(&mut it);
                    result.push(LexItem::Reference(reference));
                }
                '#' => {
                    it.next();
                    let id = get_id(&mut it);
                    result.push(LexItem::Identifier(id));
                }

                '{' => {
                    result.push(LexItem::OpenBracket);
                    it.next();
                }
                '}' => {
                    result.push(LexItem::CloseBracket);
                    it.next();
                }
                ':' => {
                    result.push(LexItem::Colon);
                    it.next();
                }
                '\n' => {
                    result.push(LexItem::EOL);
                    it.next();
                }
                ' ' => {
                    it.next();
                }
                '"' => {
                    it.next();
                    let quoted = get_quoted_string(&mut it);
                    result.push(LexItem::String(quoted));
                }
                '\'' => {
                    it.next();
                    let quoted = get_single_quoted_string(&mut it);
                    result.push(LexItem::String(quoted));
                }

                ',' => {
                    result.push(LexItem::Comma);
                    it.next();
                }
                '(' => {
                    result.push(LexItem::OpenPar);
                    it.next();
                }
                ')' => {
                    result.push(LexItem::ClosePar);
                    it.next();
                }
                '+' => {
                    result.push(LexItem::Plus);
                    it.next();
                }
                '-' => {
                    result.push(LexItem::Minus);
                    it.next();
                }
                '/' => {
                    result.push(LexItem::Div);
                    it.next();
                }
                '*' => {
                    result.push(LexItem::Mul);
                    it.next();
                }
                '=' => {
                    result.push(LexItem::Equal);
                    it.next();
                }
                '<' => {
                    result.push(LexItem::LessThan);
                    it.next();
                }
                '>' => {
                    result.push(LexItem::MoreThan);
                    it.next();
                }
                '%' => {
                    result.push(LexItem::Percent);
                    it.next();
                }
                _ => {
//                    println!("Unknown parsing {}", c);
                    it.next();
                }
            }
        }
        Ok(result)
    }
}

fn get_identifier<T: Iterator<Item=char>>(c: char, iter: &mut Peekable<T>) -> String {
    let mut identifier = String::new();
    identifier.push(c);
    while let Some(&ch) = iter.peek() {
        match ch {
            'a'...'z' | 'A'...'Z' | '_' | '0'...'9' => {
                identifier.push(ch);
                iter.next();
            }
            _ => { break; }
        }
    }
    identifier
}

fn get_id<T: Iterator<Item=char>>(iter: &mut Peekable<T>) -> String {
    let mut identifier = String::new();
    while let Some(&ch) = iter.peek() {
        match ch {
            'a'...'z' | 'A'...'Z' | '_' | '0'...'9' => {
                identifier.push(ch);
                iter.next();
            }
            _ => { break; }
        }
    }
    identifier
}


fn get_number<T: Iterator<Item=char>>(c: char, iter: &mut Peekable<T>) -> (f64, String) {
    let mut number = String::new();
    number.push(c);
    while let Some(&ch) = iter.peek() {
        match ch {
            '0'...'9' | '.' => {
                number.push(ch);
                iter.next();
            }
            _ => { break; }
        }
    }
    let parsed = number.parse::<f64>().unwrap();

    (parsed, number)
}

fn get_reference<T: Iterator<Item=char>>(iter: &mut Peekable<T>) -> String {
    let mut reference = String::new();
    while let Some(&ch) = iter.peek() {
        match ch {
            'a'...'z' | 'A'...'Z' | '_' | '0'...'9' | '.' => {
                reference.push(ch);
                iter.next();
            }
            _ => { break; }
        }
    }
    reference
}


fn get_quoted_string<T: Iterator<Item=char>>(iter: &mut Peekable<T>) -> String {
    let mut quoted = String::new();
    while let Some(&ch) = iter.peek() {
        match ch {
            '"' => {
                iter.next();
                break;
            }
            _ => {
                quoted.push(ch);
                iter.next();
            }
        }
    }
    quoted
}

fn get_single_quoted_string<T: Iterator<Item=char>>(iter: &mut Peekable<T>) -> String {
    let mut quoted = String::new();
    while let Some(&ch) = iter.peek() {
        match ch {
            '\'' => {
                iter.next();
                break;
            }
            _ => {
                quoted.push(ch);
                iter.next();
            }
        }
    }
    quoted
}


#[cfg(test)]
mod test {
    use lex::Lexer;

    #[test]
    fn simple_lex() {
        let cdl = "widget kpi {
    label : \"Label\"
}".to_string();
        let lexer = Lexer::new(cdl);
        let res = lexer.lex();
        let lex_items = res.unwrap();
        assert_eq!(lex_items.len(), 9);
    }

    #[test]
    fn lex_reference() {
        let cdl = "widget kpi @default {
    label : \"Label\"
}".to_string();
        let lexer = Lexer::new(cdl);
        let res = lexer.lex();
        let lex_items = res.unwrap();
        assert_eq!(lex_items.len(), 10);
    }

    #[test]
    fn lex_id() {
        let cdl = "widget kpi #id {
    label : \"Label\"
}".to_string();
        let lexer = Lexer::new(cdl);
        let res = lexer.lex();
        let lex_items = res.unwrap();
        assert_eq!(lex_items.len(), 10);
    }

    #[test]
    fn lex_extended() {
        let cdl = "widget kpi @default {
    label : a(b+c)
}".to_string();
        let lexer = Lexer::new(cdl);
        let res = lexer.lex();
        let lex_items = res.unwrap();
        assert_eq!(lex_items.len(), 15);
    }

    #[test]
    fn lex_advanced_expr() {
        let cdl = "value: MAX(survey:Q2,survey:interview_start=max(survey:interview_start))
        value: average(score(survey:Q7), @cr.currentPeriodB2b)
        thresholds: #82D854 >= 100%, #FFBD5B >= 80%, #FA5263 < 80%
        riskValue: IIF(average(SCORE(survey:Q1))<7,'H!',IIF(average(SCORE(survey:Q1))>8,'L',IIF(COUNT(survey:responseid)<1,'U','M')))".to_string();
        let lexer = Lexer::new(cdl);
        let res = lexer.lex();
        let lex_items = res.unwrap();
        assert_eq!(lex_items.len(), 104);
    }
}