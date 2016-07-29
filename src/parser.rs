use dom;

pub struct Parser {
    pub html_content: String,
    current_content: Vec<char>,
}

impl Parser {
    pub fn new(full_html: String) -> Parser {
        Parser {
            current_content: full_html.chars().collect(),
            html_content: full_html,
        }
    }

    // TODO check tags match, deal with self closing tags
    pub fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();

        while self.has_chars() {
            self.consume_while(char::is_whitespace);
            if self.has_chars() && self.peek() == '<' {
                self.consume();
                if self.has_chars() && self.peek() == '/' {
                    break;
                }
                nodes.push(self.parse_node())
            }
            if self.has_chars() {
                self.consume();
            }
        }
        nodes
    }

    fn parse_node(&mut self) -> dom::Node {
        // is an opening tag
        let tagname = self.consume_while(|x| x.is_digit(36));
        let attributes = self.parse_attributes();

        let mut elem = dom::element_node(tagname, attributes, Vec::new());
        elem.children = self.parse_nodes();
        elem
    }

    // Enforces the string still has characters in it.
    fn has_chars(&mut self) -> bool {
        return self.current_content.len() > 0;
    }

    // Won't panic if only called after has_chars is tested.
    fn peek(&mut self) -> char {
        self.current_content[0]
    }

    // Won't panic if only called after has_chars is tested.
    fn consume(&mut self) -> char {
        self.current_content.remove(0)
    }

    // Won't panic if only called after has_chars is tested.
    fn consume_while<F>(&mut self, condition: F) -> String 
        where F : Fn(char) -> bool {
            let mut result = String::new();
            while self.has_chars() && condition(self.peek()) {
                result.push(self.consume());
            }
            result
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        //TODO normalize caps
        let mut attributes = dom::AttrMap::new();

        while self.has_chars() && self.peek() != '>' {
            self.consume_while(char::is_whitespace);
            let name = self.consume_while(is_valid_attr_name);
            self.consume_while(char::is_whitespace);

            if self.has_chars() {
                if self.peek() == '=' {
                    self.consume(); // consume equals sign
                    let value = self.parse_attr_value();
                    attributes.insert(name, value);
                } else if self.peek() == '>' || is_valid_attr_name(self.peek()) {
                    // new attribute hash with name -> ""
                    attributes.insert(name, "".to_string());
                } else {
                    // invalid attribute name consume until whitespace or end
                    self.consume_while(|x| !x.is_whitespace() || x != '>');
                }
            }
            self.consume_while(char::is_whitespace);
        }

        if self.has_chars() && self.peek() == '>' {
            self.consume();
        }

        attributes
    }

    fn parse_attr_value(&mut self) -> String {
        self.consume_while(char::is_whitespace);
        let result = match self.consume() {
            c @ '"'| c @ '\'' => self.consume_while(|x| x != c && x != '>'),
            _ => self.consume_while(is_valid_attr_value)
        };
        if self.has_chars() {
            match self.peek() {
                '"'|'\'' => { self.consume(); },
                _ => { }
            }
        }
        result
    }
}

fn is_valid_attr_name(character: char) -> bool {
    // TODO deal with control characters
    // TODO  U+0020 SPACE, "tab" (U+0009), "LF" (U+000A), "FF" (U+000C), and "CR" (U+000D). instead of ' '
    match character {
        ' '|'"'|'\''|'>'|'/'|'=' => false,
        _ => true
    }
}

fn is_valid_attr_value(character: char) -> bool {
    // TODO no ambiguous ampersand
    match character {
        ' '|'"'|'\''|'<'|'>'|'`' => false,
        _ => true
    }
}

//TODO parse text/comment nodes vs element node