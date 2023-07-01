enum Token {
    StartTag(String),
    EndTag(String),
    Text(String),
}

struct HTMLParser {
    buffer: String,
}

impl HTMLParser {
    fn new() -> Self {
        HTMLParser {
            buffer: String::new(),
        }
    }

    fn parse(&mut self, html: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        for c in html.chars() {
            match c {
                '<' => {
                    if !self.buffer.is_empty() {
                        self.consume_text_token(&mut tokens);
                    }
                }
                '>' => {
                    if !self.buffer.is_empty() {
                        if self.buffer.starts_with('/') {
                            tokens.push(Token::EndTag(self.buffer[1..].to_string()));
                        } else {
                            tokens.push(Token::StartTag(self.buffer.clone()));
                        }
                        self.buffer.clear();
                    }
                }
                _ => self.buffer.push(c),
            }
        }
        if !self.buffer.is_empty() {
            self.consume_text_token(&mut tokens);
        }
        tokens
    }

    fn consume_text_token(&mut self, tokens: &mut Vec<Token>) {
        let trimmed_buffer = self.buffer.trim().to_string();
        if !trimmed_buffer.is_empty() {
            tokens.push(Token::Text(trimmed_buffer));
        }
        self.buffer.clear();
    }
}


fn main() {
    let html = r#"
        <html>
            <head>
                <title>Basic Parser</title>
            </head>
            <body>
                <h1>Test</h1>
                <p>Should be detected as text.</p>
            </body>
        </html>
    "#;

    let mut parser = HTMLParser::new();
    let tokens = parser.parse(html);

    for token in tokens {
        match token {
            Token::StartTag(tag_name) => println!("Start tag: {}", tag_name),
            Token::EndTag(tag_name) => println!("End tag: {}", tag_name),
            Token::Text(text) => println!("Text: {}", text),
        }
    }
}
