use wasm_bindgen::prelude::wasm_bindgen;

use crate::Game;

#[wasm_bindgen]
impl Game {
    pub fn get_color(id: u8, alpha: Option<f32>) -> String {
        let id = id as u16;
        // note: there are 300 colors by this formula
        let rank = id / 30;
        let h = id % 30;

        let r = rank % 10;
        let s = 5 * r + 50;
        let l = s;
        let h = h * 10 + 20;

        if let Some(alpha) = alpha {
            format!("hsla({h},{s}%,{l}%,{alpha})")
        } else {
            format!("hsl({h},{s}%,{l}%)")
        }
    }
}
