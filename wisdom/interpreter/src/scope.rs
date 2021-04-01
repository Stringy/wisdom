use std::collections::HashMap;
use ast2::Value;
use std::cell::RefCell;

pub type Scope = HashMap<String, Value>;

pub struct Context {
    /// This is treated as a stack, and is used
    /// for lookup of named values
    scopes: RefCell<Vec<Scope>>
}

impl Context {
    pub fn new() -> Self {
        let scopes = vec![Scope::new()];
        Self {
            scopes: RefCell::new(scopes),
        }
    }

    ///
    /// Push a new scope to the context, so that subsequent
    /// variable storage will be placed in this new scope.
    ///
    /// Using this mechanism, variables can be shadowed in
    /// deeper scopes.
    ///
    pub fn push(&self) {
        self.scopes.borrow_mut().push(Scope::new())
    }

    ///
    /// Pops the top scope from the stack, discarding all
    /// stored variables within.
    ///
    /// It is discarded, rather than returned because the values
    /// are now dropped.
    ///
    pub fn pop(&self) {
        self.scopes.borrow_mut().pop();
    }

    ///
    /// Lookup a value from within the current context. It searches
    /// backwards up the Scope stack to find the first occurrence of
    /// the name.
    ///
    pub fn lookup(&self, name: &String) -> Option<Value> {
        for scope in self.scopes.borrow().iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v.to_owned());
            }
        }
        None
    }

    ///
    /// Stores a value associated with a name into the top scope
    /// on the stack.
    ///
    pub fn store(&self, name: String, value: Value) {
        if let Some(_) = self.lookup(&name) {
            for scope in self.scopes.borrow_mut().iter_mut().rev() {
                if let Some(_) = scope.get_mut(&name) {
                    scope.insert(name.to_owned(), value.to_owned());
                }
            }
        } else {
            let mut scopes = self.scopes.borrow_mut();
            let end = scopes.len();
            let scope = scopes.get_mut(end - 1);
            match scope {
                Some(scope) => {
                    scope.insert(name, value);
                }
                None => panic!("there should always be at least one scope")
            }
        }
    }
}
