use ron;
use pretty_assertions::assert_eq;

use paste::item;

use ron::ser::PrettyConfig;

use ast::Stmt;
use tokenizer::{TokenStream, FromTokens};

macro_rules! test_ast_creation {
    ($name:ident, $path:literal) => {
        item! {
            #[test]
            fn [< test_ $name >]() {
                let script = std::fs::read_to_string($path).unwrap();
                let tokens = TokenStream::new(&script);
                let mut stmts = Vec::new();
                while !tokens.is_empty() {
                    stmts.push(Stmt::from_tokens(&tokens).unwrap());
                }
                let expected = std::fs::read_to_string(format!("{}.ron", $path)).unwrap();
                let expected = expected.trim_end();
                assert_eq!(ron::ser::to_string_pretty(&stmts, PrettyConfig::new()).unwrap(), expected);
            }
        }
    }
}

test_ast_creation!(simple_expr, "tests/data/simple_expr.wis");
test_ast_creation!(complex_expr, "tests/data/complex_expr.wis");
test_ast_creation!(simple_multiline, "tests/data/simple_multiline.wis");
test_ast_creation!(while, "tests/data/while.wis");
test_ast_creation!(if, "tests/data/if.wis");
test_ast_creation!(func, "tests/data/func.wis");
test_ast_creation!(multi_op_expr, "tests/data/multi-op-expr.wis");
