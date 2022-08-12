(async function init() {
    console.log("initializing worker");

    let wasm_bindgen = await import('./pkg/backend/index.js');

    self.onmessage = async e => {
        var worker_result = wasm_bindgen.double(e.data);
        self.postMessage(worker_result);
    };

    console.log("worker initialized");
})();
