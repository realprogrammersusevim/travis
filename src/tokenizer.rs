use arrayvec::ArrayString;

pub trait Tokenizer<'a> {
    /// A Tokenizer always needs to produce an Iterator of Tokens.
    type TokenIter: Iterator<Item = Token>;

    /// Takes the input string and tokenizes it based on the implementations rules.
    fn tokenize(&self, input: &'a str) -> Self::TokenIter;
}

const MAX_STACK_TERM_LEN: usize = 15;

enum Term {
    Stack(ArrayString<MAX_STACK_TERM_LEN>),
    Heap(String),
}

pub struct Token {
    term: Term,
    start_offset: usize,
    position: usize,
}

impl Token {
    #[inline]
    pub fn from_str(term: &str, start_offset: usize, position: usize) -> Self {
        Token {
            term: Token::convert_term(term),
            start_offset: start_offset,
            position: position,
        }
    }

    #[inline]
    fn convert_term(term: &str) -> Term {
        if term.len() <= MAX_STACK_TERM_LEN {
            Term::Stack(ArrayString::<MAX_STACK_TERM_LEN>::from(term).unwrap())
        } else {
            Term::Heap(term.to_string())
        }
    }

    #[inline]
    pub fn term(&self) -> &str {
        match self.term {
            Term::Heap(ref s) => s.as_ref(),
            Term::Stack(ref s) => s.as_ref(),
        }
    }
}

pub struct CharTokenIter<'a> {
    filter: fn(&(usize, (usize, char))) -> bool,
    input: &'a str,
    byte_offset: usize,
    char_offset: usize,
    position: usize,
}

impl<'a> CharTokenIter<'a> {
    pub fn new(filter: fn(&(usize, (usize, char))) -> bool, input: &'a str) -> Self {
        CharTokenIter {
            filter: filter,
            input: input,
            byte_offset: 0,
            char_offset: 0,
            position: 0,
        }
    }
}

impl<'a> Iterator for CharTokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let mut skipped_bytes = 0;
        let mut skipped_chars = 0;
        for (cidx, (bidx, c)) in self.input[self.byte_offset..]
            .char_indices()
            .enumerate()
            .filter(&self.filter)
        {
            let char_len = c.len_utf8();
            if cidx - skipped_chars == 0 {
                self.byte_offset = self.byte_offset + char_len;
                self.char_offset += 1;
                skipped_bytes = skipped_bytes + char_len;
                skipped_chars += 1;
                continue;
            }

            let slice = &self.input[self.byte_offset..self.byte_offset + bidx - skipped_bytes];
            let token = Token::from_str(slice, self.char_offset, self.position);
            self.char_offset = self.char_offset + slice.chars().count() + 1;
            self.position += 1;
            self.byte_offset = self.byte_offset + bidx + char_len - skipped_bytes;
            return Some(token);
        }

        if self.byte_offset < self.input.len() {
            let slice = &self.input[self.byte_offset..];
            let token = Token::from_str(slice, self.char_offset, self.position);
            self.byte_offset = self.input.len();
            Some(token)
        } else {
            None
        }
    }
}

pub struct WhitespaceTokenizer;

impl<'a> Tokenizer<'a> for WhitespaceTokenizer {
    type TokenIter = CharTokenIter<'a>;

    fn tokenize(&self, input: &'a str) -> Self::TokenIter {
        CharTokenIter::new(is_whitespace, input)
    }
}

#[inline]
fn is_whitespace(input: &(usize, (usize, char))) -> bool {
    let (_, (_, c)) = *input;
    c.is_whitespace()
}

#[test]
fn should_split_between_words() {
    let expected = vec![
        Token::from_str("hello", 0, 0),
        Token::from_str("world", 6, 1),
    ];
    let actually = WhitespaceTokenizer
        .tokenize("hello world")
        .collect::<Vec<Token>>();
    assert_eq!(expected, actually);
}

#[test]
fn should_handle_mixed_chars() {
    let expected = vec![
        Token::from_str("abc界", 0, 0),
        Token::from_str("abc界", 5, 1),
    ];
    let actually = WhitespaceTokenizer
        .tokenize("abc界 abc界")
        .collect::<Vec<Token>>();
    assert_eq!(expected, actually);
}
