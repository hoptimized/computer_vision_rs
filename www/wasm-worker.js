(async function init() {
    console.log("initializing worker");

    let wasm_bindgen = await import('./pkg/backend/index.js');
    let initialized = wasm_bindgen.default();

    self.onmessage = async e => {
        await initialized;
        console.log(e.data);
        var worker_result = wasm_bindgen.double(e.data);
        console.log(worker_result);
        self.postMessage(worker_result);
    };

    console.log("worker initialized");
})();
