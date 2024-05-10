import init, { greet } from '../../pkg/form_builder.js';

export async function loadWasm() {
    await init();
    document.getElementById('greeting').textContent = greet('it Worked');
}

window.loadWasm = loadWasm;
