use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub enum Button {
  A,
  B,
  SELECT,
  START,
  RIGHT,
  LEFT,
  UP,
  DOWN,
}

#[wasm_bindgen(getter_with_clone)]
pub struct CartridgeInfo {
  pub title: String,
  pub licensee: String,
  pub cartridge_type: String,
  pub rom_size: String,
  pub ram_size: String,
  pub cgb_mode: String
}

#[wasm_bindgen]
pub struct OAMObject {
  pub lcd_y: u8,
  pub lcd_x: u8,
  pub tile_index: u8,
  pub attributes: u8,
}

#[wasm_bindgen]
pub struct CPUInfo {
  pub af: u16,
  pub bc: u16,
  pub de: u16,
  pub hl: u16,
  pub sp: u16,
  pub pc: u16,
  pub stopped: bool,
  pub enabled: bool,
}