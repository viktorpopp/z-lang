use crate::scanner::token::TokenKind;
use std::{collections::HashMap, sync::LazyLock};

pub static KEYWORDS: LazyLock<HashMap<&str, TokenKind>> = LazyLock::new(|| {
    let mut keywords: HashMap<&str, TokenKind> = HashMap::new();
    keywords.insert("fn", TokenKind::Fn);
    keywords.insert("return", TokenKind::Return);
    keywords
});
