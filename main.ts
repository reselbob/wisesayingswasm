const wasmCode = await Deno.readFile("./target/wasm32-unknown-unknown/debug/wisesayings.wasm");
const wasmModule = new WebAssembly.Module(wasmCode);
const wasmInstance = new WebAssembly.Instance(wasmModule);

const saying = wasmInstance.exports.get_wise_saying as CallableFunction

console.log(saying());
console.log(saying());
console.log(saying());
console.log(saying());
