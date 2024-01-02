//! Each clock cycle ends up taking 280 microseconds. (3.671 kHz)

#![forbid(unsafe_code)]

use wasm_bindgen::prelude::*;

mod rom70packed;

#[wasm_bindgen]
pub async fn run() {
  hp_classic::run::<10>(rom70packed::ROM.to_vec()).await
}
