use std::cell::RefCell;
use std::collections::HashMap;

use ast::Value;

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
    /// Allows the caller to run a function within a new scope and to ensure that
    /// as soon as the scope has ended, it is popped from this context.
    ///
    pub fn scoped<R, E>(&self, func: impl Fn() -> Result<R, E>) -> Result<R, E> {
        self.push();
        let result = func();
        self.pop();
        result
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
    /// Checks whether a given name exists in the context
    ///
    pub fn exists(&self, name: &String) -> bool {
        self.lookup(name).is_some()
    }

    ///
    /// Stores a variable and its value into the scope. If it exists
    /// in the context, it is overwritten with the new value.
    /// If it doesn't exist, then it is added to the top scope.
    ///
    pub fn store(&self, name: String, value: Value) {
        if let Some(_) = self.lookup(&name) {
            for scope in self.scopes.borrow_mut().iter_mut().rev() {
                if let Some(_) = scope.get_mut(&name) {
                    scope.insert(name.to_owned(), value.to_owned());
                }
            }
        } else {
            self.store_top(name, value)
        }
    }

    ///
    /// Inserts a new variable in the top scope, most useful
    /// for pushing function arguments.
    ///
    pub fn store_top(&self, name: String, value: Value) {
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
