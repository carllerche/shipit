use shipit::github::{Client, Error};
use serde::{Serialize};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::mem;

pub struct Builder {
    transport: MockTransport,
}

pub struct MockTransport {
    responses: RefCell<VecDeque<Value>>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            transport: MockTransport::new(),
        }
    }

    pub fn response(&mut self, value: Value) -> &mut Self {
        self.transport
            .responses
            .borrow_mut()
            .push_front(value);

        self
    }

    pub fn build(&mut self) -> Client<MockTransport> {
        let transport = mem::replace(&mut self.transport, MockTransport::new());
        Client::with_transport(transport)
    }
}

impl MockTransport {
    fn new() -> MockTransport {
        MockTransport {
            responses: RefCell::new(VecDeque::new()),
        }
    }
}

impl Transport for MockTransport {
    fn query<T, U>(&self, _query: &T) -> Result<U, Error>
    where
        T: Serialize,
        U: DeserializeOwned
    {
        use serde::Deserialize;

        assert!(
            !self.responses.borrow().is_empty(),
            "not expecting Github query");

        // TODO: validate the query
        let value = self.responses
            .borrow_mut()
            .pop_front()
            .unwrap();

        Deserialize::deserialize(value)
            .map_err(Into::into)
    }
}
