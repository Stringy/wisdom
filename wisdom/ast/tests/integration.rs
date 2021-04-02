use serde_json;

use paste::item;

use ast::Stmt;
use tokenizer::{TokenStream, FromTokens};

macro_rules! test_ast_creation {
    ($name:ident, $path:literal) => {
        item! {
            #[test]
            fn [< test_ $name >]() {
                let script = std::fs::read_to_string(format!("{}.wis", $path)).unwrap();
                let tokens = TokenStream::new(&script);
                let mut stmts = Vec::new();
                while !tokens.is_empty() {
                    stmts.push(Stmt::from_tokens(&tokens).unwrap());
                }
                let expected = std::fs::read_to_string(format!("{}.json", $path)).unwrap();
                assert_eq!(expected, serde_json::to_string_pretty(&stmts).unwrap());
            }
        }
    }
}

test_ast_creation!(simple_expr, "tests/data/simple_expr");
test_ast_creation!(complex_expr, "tests/data/complex_expr");
test_ast_creation!(simple_multiline, "tests/data/simple_multiline");
test_ast_creation!(while, "tests/data/while");
test_ast_creation!(if, "tests/data/if");
test_ast_creation!(func, "tests/data/func");