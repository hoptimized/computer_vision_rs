async function init_frontend() {
    console.debug("loading wasm…");

    import('./pkg/frontend')
        .then(on_wasm_loaded)
        .catch(on_wasm_error);

    function on_wasm_loaded(wasm_bindgen) {
        console.debug("wasm loaded. starting app…");

        // This call installs a bunch of callbacks and then returns:
        wasm_bindgen.start("the_canvas_id");

        console.debug("app started.");
        document.getElementById("center_text").remove();
    }

    function on_wasm_error(error) {
        console.error("Failed to start: " + error);
        document.getElementById("center_text").innerHTML = `
        <p>
            An error occurred during loading:
        </p>
        <p style="font-family:Courier New">
            ${error}
        </p>
        <p style="font-size:14px">
            Make sure you use a modern browser with WebGL and WASM enabled.
        </p>`;
    }
}

async function init_backend() {
    console.log("initializing worker thread");

    let worker = new Worker(
        new URL('./wasm-worker.js', import.meta.url),
        { type: 'module' }
    );

    worker.onmessage = (e) => {
        console.log(e.data);
    }

    console.log("worker thread initialized");

    worker.postMessage(8);
}

(async function init() {
    await init_backend();
    await init_frontend();
})();
