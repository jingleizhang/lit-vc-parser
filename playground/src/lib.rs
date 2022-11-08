extern crate vc_parser;
extern crate wasm_bindgen;
use vc_parser::assertions::compile::Compiling;
use vc_parser::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub fn compute(e: String) -> (Result<Compiling, Error>, Result<i32, Error>) {
    let compiling = compile_vc(e.as_str());
    let running = compiling
        .clone()
        .and_then(|c| run_vc(e.as_str(), Some(c.clone())));
    (compiling, running)

    //let compiling = compile(e.as_str());
    // let running = compiling
    //     .clone()
    //     .and_then(|c| run(e.as_str(), Some(c.clone())));
    // (compiling, running)

}

#[wasm_bindgen]
pub fn compute_and_represent(e: String) -> JsValue {
    //compile_vc
    let results = compute(e);
    let compiling_r = results.0.map(|c| CompilingRepresent::from_compiling(&c));
    let re = (compiling_r, results.1);
    JsValue::from_serde(&re).unwrap()
}
