## Wisdom

Wisdom is a scripting language that aims to be as good at 
text processing as Perl, with the readability of languages such as
Python or Rust.

To this end, it incorporates the dynamic typing of Python with the
regex support of Perl some of the syntax of Rust. Eventually
the language should look something like the following:

```wisdom
use std::fs;

fn filter(path: str, pattern: regex) {
    if (path ~= pattern) {
        print("Found: ${path}")
    }
}

fn main() {
    for file in std::fs::walk() {
        filter(path, "^[foo].*$");
    }
}
``` 

Currently, Wisdom is in a very early state where I'm working
on tokenizing the source code, creating the AST and building a simple
REPL. Features will be added and modified continuously throughout
this period, so nothing should be considered stable at this point.

Current Roadmap:

- [x] Simple maths expressions
- [x] Recursive expressions
- [x] Variables, bindings, and their inclusion in expressions.
- [x] Expand literal kinds
- [x] Expand expressions, inc boolean operations.
- [x] String literals, including appropriate escapes
- [x] Statements
- [ ] built in functions
    - [x] print
    - [ ] ... others
- [ ] User defined Functions
- [ ] Classes maybe?
- [ ] A proper interpreter / JIT maybe?
- [ ] World domination