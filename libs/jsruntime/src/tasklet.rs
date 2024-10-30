use std::collections::VecDeque;
use std::num::NonZeroU32;

use rustc_hash::FxHashMap;

use crate::llvmir::Coroutine;
use crate::llvmir::CoroutineStatus;
use crate::Runtime;
use crate::Value;

impl<X> Runtime<X> {
    /// Perform all tasklets.
    pub fn run(&mut self) {
        while let Some(msg) = self.tasklet_system.next_msg() {
            self.handle_message(msg);
        }
    }

    fn handle_message(&mut self, msg: Message) {
        crate::logger::debug!(event = "handle_message", ?msg);
        match msg {
            Message::PromiseResolved {
                promise_id,
                ref result,
            } => self.process_promise(promise_id, result, &Value::NONE),
            Message::PromiseRejected {
                promise_id,
                ref error,
            } => self.process_promise(promise_id, &Value::NONE, error),
        }
    }

    // promise

    pub fn register_promise(&mut self, coroutine: *mut Coroutine) -> PromiseId {
        crate::logger::debug!(event = "register_promise", ?coroutine);
        self.tasklet_system.register_promise(coroutine)
    }

    pub fn await_promise(&mut self, promise_id: PromiseId, awaiting: PromiseId) {
        crate::logger::debug!(event = "await_promise", ?promise_id, ?awaiting);
        self.tasklet_system.await_promise(promise_id, awaiting);
    }

    pub fn process_promise(&mut self, promise_id: PromiseId, result: &Value, error: &Value) {
        crate::logger::debug!(event = "process_promise", ?promise_id, ?result, ?error);
        let coroutine = self.tasklet_system.get_coroutine(promise_id);
        match Coroutine::resume(self.as_void_ptr(), coroutine, promise_id, result, error) {
            CoroutineStatus::Done(result) => {
                self.tasklet_system.resolve_promise(promise_id, result)
            }
            CoroutineStatus::Error(error) => self.tasklet_system.reject_promise(promise_id, error),
            CoroutineStatus::Suspend => (),
        }
    }

    pub fn emit_promise_resolved(&mut self, promise_id: PromiseId, result: Value) {
        self.tasklet_system
            .emit_promise_resolved(promise_id, result);
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PromiseId(NonZeroU32);

impl From<u32> for PromiseId {
    fn from(value: u32) -> Self {
        Self(NonZeroU32::new(value).unwrap())
    }
}

impl From<PromiseId> for u32 {
    fn from(value: PromiseId) -> Self {
        value.0.get()
    }
}

pub struct System {
    messages: VecDeque<Message>,
    promises: FxHashMap<PromiseId, Promise>,
    next_promise_id: u32,
}

impl System {
    pub fn new() -> Self {
        Self {
            messages: Default::default(),
            promises: Default::default(),
            next_promise_id: 1,
        }
    }

    // promises

    fn register_promise(&mut self, coroutine: *mut Coroutine) -> PromiseId {
        let promise_id = PromiseId(NonZeroU32::new(self.next_promise_id).unwrap());
        self.promises.insert(promise_id, Promise::new(coroutine));
        self.next_promise_id += 1;
        promise_id
    }

    fn await_promise(&mut self, promise_id: PromiseId, awaiting: PromiseId) {
        debug_assert!(self.promises.contains_key(&promise_id));
        debug_assert!(self.promises.contains_key(&awaiting));
        let promise = self.promises.get_mut(&promise_id).unwrap();
        debug_assert!(promise.awaiting.is_none());
        match promise.state {
            PromiseState::Pending => promise.awaiting = Some(awaiting),
            PromiseState::Resolved(result) => {
                self.emit_promise_resolved(awaiting, result);
                self.promises.remove(&promise_id);
            }
            PromiseState::Rejected(error) => {
                self.emit_promise_rejected(awaiting, error);
                self.promises.remove(&promise_id);
            }
        }
    }

    fn get_coroutine(&self, promise_id: PromiseId) -> *mut Coroutine {
        self.promises.get(&promise_id).unwrap().coroutine
    }

    fn emit_promise_resolved(&mut self, promise_id: PromiseId, result: Value) {
        crate::logger::debug!(event = "emit_promise_resolved", ?promise_id, ?result);
        self.messages
            .push_back(Message::PromiseResolved { promise_id, result });
    }

    fn emit_promise_rejected(&mut self, promise_id: PromiseId, error: Value) {
        crate::logger::debug!(event = "emit_promise_rejected", ?promise_id, ?error);
        self.messages
            .push_back(Message::PromiseRejected { promise_id, error });
    }

    fn next_msg(&mut self) -> Option<Message> {
        self.messages.pop_front()
    }

    fn resolve_promise(&mut self, promise_id: PromiseId, result: Value) {
        crate::logger::debug!(event = "resolve_promise", ?promise_id, ?result);
        let promise = self.promises.get_mut(&promise_id).unwrap();
        debug_assert!(matches!(promise.state, PromiseState::Pending));
        if let Some(awaiting) = promise.awaiting {
            self.promises.remove(&promise_id);
            self.emit_promise_resolved(awaiting, result);
        } else {
            promise.state = PromiseState::Resolved(result);
        }
    }

    fn reject_promise(&mut self, promise_id: PromiseId, error: Value) {
        crate::logger::debug!(event = "reject_promise", ?promise_id, ?error);
        let promise = self.promises.get_mut(&promise_id).unwrap();
        debug_assert!(matches!(promise.state, PromiseState::Pending));
        if let Some(awaiting) = promise.awaiting {
            self.promises.remove(&promise_id);
            self.emit_promise_rejected(awaiting, error);
        } else {
            promise.state = PromiseState::Rejected(error);
        }
    }
}

// messages

#[derive(Debug)]
enum Message {
    PromiseResolved {
        promise_id: PromiseId,
        result: Value,
    },
    PromiseRejected {
        promise_id: PromiseId,
        error: Value,
    },
}

// promise

// TODO: should the coroutine be separated from the promise?
struct Promise {
    // TODO(issue#237): GcCellRef
    coroutine: *mut Coroutine,
    awaiting: Option<PromiseId>,
    state: PromiseState,
}

impl Promise {
    fn new(coroutine: *mut Coroutine) -> Self {
        Self {
            coroutine,
            awaiting: None,
            state: PromiseState::Pending,
        }
    }
}

enum PromiseState {
    Pending,
    Resolved(Value),
    Rejected(Value),
}
