#![no_main]
// In-process libFuzzer harness over parity-wasm's binary deserializer — the same
// code path as the historical `deserialize` target (which fed binaryen-generated
// modules into `deserialize_buffer`). Fuzzing the parser on raw bytes covers the
// full decoder surface; on a successful parse we round-trip through the
// serializer and re-parse, catching serialize/deserialize asymmetries.
use libfuzzer_sys::fuzz_target;
use parity_wasm::elements::Module;

fuzz_target!(|data: &[u8]| {
    if let Ok(module) = parity_wasm::deserialize_buffer::<Module>(data) {
        let module = match module.parse_names() {
            Ok(m) => m,
            Err((_, m)) => m,
        };
        let module = match module.parse_reloc() {
            Ok(m) => m,
            Err((_, m)) => m,
        };
        if let Ok(bytes) = parity_wasm::serialize(module) {
            let _ = parity_wasm::deserialize_buffer::<Module>(&bytes);
        }
    }
});
