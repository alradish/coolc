use crate::lexical_structure::Token;

pub fn match_token(string: &str) -> Token {
    match string {
        "class" => Token::Class,
        "if" => Token::If,
        "then" => Token::Then,
        "else" => Token::Else,
        "fi" => Token::Fi,
        "inherits" => Token::Inherits,
        "in" => Token::In,
        "isVoid" => Token::IsVoid,
        "let" => Token::Let,
        "loop" => Token::Loop,
        "pool" => Token::Pool,
        "while" => Token::While,
        "case" => Token::Case,
        "esac" => Token::Esac,
        "new" => Token::New,
        "of" => Token::Of,
        "not" => Token::Not,

        // "" => Token::BoolConst,
        // "" => Token::TypeID,
        // "" => Token::ObjectID,
        // "" => Token::IntConst,
        // "" => Token::StrConst,

        "(" => Token::OpenRoundBrace,
        ")" => Token::CloseRoundBrace,
        "{" => Token::OpenCurlyBrace,
        "}" => Token::CloseCurlyBrace,
        ":" => Token::Colon,
        ";" => Token::Semicolon,
        "." => Token::Dot,
        "," => Token::Comma,
        "=" => Token::Assign,
        "==" => Token::Equal,
        "+" => Token::Plus,
        "-" => Token::Minus,
        "*" => Token::Multiply,
        "/" => Token::Divide,
        "<" => Token::Less,
        "<=" => Token::LessOrEqual,
        "->" => Token::Darrow,
        "@" => Token::AtSign,
        "~" => Token::Tilde,
        _ => Token::Error,
    }
}