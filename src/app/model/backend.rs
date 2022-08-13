use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{console, MessageEvent, Worker};

pub struct Backend {
    worker: Worker,
    persistent_callback_handle: Closure<dyn FnMut(MessageEvent)>,
}

impl Backend {
    pub fn new() -> Self {
        let worker = Worker::new("./wasm-worker.js").unwrap();
        console::log_1(&"Backend worker created".into());

        let persistent_callback_handle = Backend::get_on_msg_callback();

        let res = Self {
            worker,
            persistent_callback_handle,
        };

        res.worker.set_onmessage(Some(res.persistent_callback_handle.as_ref().unchecked_ref()));

        res
    }

    pub fn double(&self, num: u32) {
        console::log_1(&"Posting a message from WASM".into());
        let _ = self.worker.post_message(&num.into());
    }

    fn get_on_msg_callback() -> Closure<dyn FnMut(MessageEvent)> {
        let callback = Closure::wrap(Box::new(move |event: MessageEvent | {
            console::log_2(&"Received response: ".into(), &event.data().into());

            // TODO: handle message
        }) as Box<dyn FnMut(_)>);

        callback
    }
}
