import * as Comlink from 'comlink';

async function initApi() {
    let workerApi = await (async () => {
        const wasm_bindgen = await import('./pkg/backend/index.js');
        await wasm_bindgen.default();

        return {
            double: (num) => wasm_bindgen.double(num),
        };
    })();

    return Comlink.proxy({
        initialized: !!workerApi,
        workerApi
    });
}

Comlink.expose({
    handle: initApi()
});
