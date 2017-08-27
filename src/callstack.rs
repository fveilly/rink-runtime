use runtime::container::Container;
use runtime::divert::PushPopType;
use runtime::RuntimeObject;
use runtime_context::RuntimeContext;
use macros;

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct Thread {
    stack: Vec<RuntimeContext>
}

impl<'ru> Thread {
    pub fn new() -> Thread {
        Thread {
            stack: Vec::new()
        }
    }

    pub fn stack(&self) -> &Vec<RuntimeContext> {
        &self.stack
    }

    pub fn push(&mut self, runtime_context: RuntimeContext) {
        self.stack.push(runtime_context);
    }

    pub fn pop(&mut self) -> Option<RuntimeContext> {
        self.stack.pop()
    }

    pub fn pop_if<F>(&mut self, f: F) -> Option<RuntimeContext>
        where F: FnOnce(&RuntimeContext) -> bool {
        let mut should_pop = match self.stack.last() {
            Some(runtime_context) => f(runtime_context),
            _ => false
        };

        // Need to do this because at the moment rust does not support Non-lexical borrow scopes
        // See https://github.com/rust-lang/rfcs/issues/811
        if should_pop {
            return self.stack.pop();
        }

        None
    }

}

#[derive(Clone)]
pub struct CallStack {
    threads: Vec<Thread>
}

impl<'ru> CallStack {
    pub fn new(root_container: &Rc<Container>) -> CallStack {
        let mut threads = Vec::new();
        let mut thread = Thread::new();

        thread.push(RuntimeContext::new(root_container));
        threads.push(thread);

        CallStack {
            threads: threads
        }
    }

    pub fn thread(&self) -> Option<&Thread> {
        self.threads.last()
    }

    pub fn stack(&self) -> Option<&Vec<RuntimeContext>> {
        self.thread().map(|thread| thread.stack())
    }

    pub fn runtime_context(&self) -> Option<&RuntimeContext> {
        self.stack().and_then(|stack| stack.last())
    }

    pub fn runtime_object(&self) -> Option<&RuntimeObject> {
        match self.runtime_context() {
            Some(runtime_context) => runtime_context.get(),
            _ => None
        }
    }

    pub fn depth(&self) -> usize {
        match self.stack() {
            Some(stack) => stack.len(),
            _ => 0
        }
    }

    pub fn thread_from_index(&self, index: usize) -> Option<&Thread> {
        self.threads.get(index)
    }

    pub fn push_thread(&mut self) -> bool {
        if let Some(thread) = self.threads.last().map(|thread| thread.clone()) {
            self.threads.push(thread);
            return true;
        }

        false
    }

    pub fn pop_thread(&mut self) -> bool {
        match self.threads.pop() {
            Some(_) => true,
            _ => false
        }
    }

    pub fn reset(&mut self, thread: Thread) {
        self.threads.clear();
        self.threads.push(thread);
    }
}