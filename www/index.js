console.debug("loading wasm…");

import('./pkg/backend')
    .then(m => m.greet('World!'))
    .catch(console.error);

import('./pkg/frontend')
  .then(on_wasm_loaded)
  .catch(on_wasm_error);

function on_wasm_loaded(wasm_bindgen) {
    console.debug("wasm loaded. starting app…");

    //wasm_bindgen.greet('WASM32!')

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
