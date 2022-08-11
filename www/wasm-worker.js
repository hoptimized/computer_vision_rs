import * as Comlink from 'comlink';
import { threads } from 'wasm-feature-detect';

async function initApi() {
    let workerApi = await (async () => {
        if (!(await threads())) return;

        const wasm_bindgen = await import('./pkg/backend/index.js');

        await wasm_bindgen.default();
        await wasm_bindgen.initThreadPool(navigator.hardwareConcurrency);

        return {
            double: (num) => wasm_bindgen.double(num),
            sum: (nums) => wasm_bindgen.sum(nums),
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
