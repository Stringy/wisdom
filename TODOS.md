# TODOs (20)
 * [welp/src/main.rs](welp/src/main.rs) (1)
   * `// TODO: support reading from file`
 * [wisdom/ast/src/error.rs](wisdom/ast/src/error.rs) (1)
   * `// TODO: make ExpectedTokens description not a debug thing`
 * [wisdom/ast/src/expr.rs](wisdom/ast/src/expr.rs) (6)
   * `// TODO: AssignOp(Expr, Expr),`
   * `// TODO: more error construction helpers would be very useful`
   * `// TODO: perhaps a literal should just contain the string repr (and move Value somewhere else)`
   * `// TODO: better way of doing this kind of literal processing?`
   * `// TODO: this will need to be tweaked when we introduce right-associative operators`
   * `// TODO: definitely need a better way of constructing these`
 * [wisdom/ast/src/func.rs](wisdom/ast/src/func.rs) (1)
   * `// TODO: add return types`
 * [wisdom/ast/src/stmt.rs](wisdom/ast/src/stmt.rs) (1)
   * `// TODO: look into semi-colon processing - when do we need them?`
 * [wisdom/ast/src/value.rs](wisdom/ast/src/value.rs) (2)
   * `// TODO: need to rethink the value thing. Not sure it should live here (interpreter maybe?)`
   * `// TODO: add some tests for all operations`
 * [wisdom/interpreter/src/error.rs](wisdom/interpreter/src/error.rs) (1)
   * `// TODO: update to use an actual position`
 * [wisdom/interpreter/src/slow.rs](wisdom/interpreter/src/slow.rs) (2)
   * `// TODO: add labels to break statements`
   * `// TODO: add labels to continue statements`
 * [wisdom/tests/slow_interpreter.rs](wisdom/tests/slow_interpreter.rs) (1)
   * `// TODO: improve integration test rig so I can add more tests more easily.`
 * [wisdom/tokenizer/src/cursor.rs](wisdom/tokenizer/src/cursor.rs) (3)
   * `/// TODO: refactor this potentially enormous and frequent copy.`
   * `/// TODO: add some tests for consume_number_literal`
   * `// TODO: is there a better way of wrapping the closure like this?`
 * [wisdom/tokenizer/src/token_stream.rs](wisdom/tokenizer/src/token_stream.rs) (1)
   * `/// TODO: remove whitespace functions?`
