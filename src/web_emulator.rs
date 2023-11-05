use std::panic;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::AudioContext;
use crate::web_audio_driver::WebAudioDriver;
use crate::canvas_renderer::CompositeCanvasRenderer;
use rustboy_core::input::Button;
use rustboy_core::cpu::CPUInfo;
use rustboy_core::emulator::Emulator;
use rustboy_core::memory::OAMObject;
use crate::bindings::Button as WebButton;
use crate::bindings::OAMObject as WebOAMObject;
use crate::bindings::CPUInfo as WebCPUInfo;
use crate::bindings::CartridgeInfo as WebCartridgeInfo;
use log::Level;
use rustboy_core::cartridge_info::CartridgeInfo;

#[wasm_bindgen]
pub struct WebEmulator {
  emulator: Emulator<WebAudioDriver, CompositeCanvasRenderer>
}

#[wasm_bindgen]
impl WebEmulator {
  pub fn new(rom_bytes: &[u8], audio_context: AudioContext) -> Self {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug).unwrap();
    let audio_driver = WebAudioDriver::new(audio_context);
    let renderer = CompositeCanvasRenderer::new();
    WebEmulator {
      emulator: Emulator::new(rom_bytes, audio_driver, renderer)
    }
  }

  fn convert_to_button(web_button: WebButton) -> Button {
    match web_button {
      WebButton::A => Button::A,
      WebButton::B => Button::B,
      WebButton::SELECT => Button::SELECT,
      WebButton::START => Button::START,
      WebButton::RIGHT => Button::RIGHT,
      WebButton::LEFT => Button::LEFT,
      WebButton::UP => Button::UP,
      WebButton::DOWN => Button::DOWN
    }
  }

  fn convert_to_web_oam_object(oam_object: OAMObject) -> WebOAMObject {
    WebOAMObject {
      lcd_y: oam_object.lcd_y,
      lcd_x: oam_object.lcd_x,
      tile_index: oam_object.tile_index,
      attributes: oam_object.attributes.value()
    }
  }

  fn convert_to_web_cpu_info(cpu_info: CPUInfo) -> WebCPUInfo {
    WebCPUInfo {
      af: cpu_info.af,
      bc: cpu_info.bc,
      de: cpu_info.de,
      hl: cpu_info.hl,
      sp: cpu_info.sp,
      pc: cpu_info.pc,
      stopped: cpu_info.stopped,
      enabled: cpu_info.enabled,

    }
  }

  fn convert_to_web_cartridge_info(cartridge_info: &CartridgeInfo) -> WebCartridgeInfo {
    WebCartridgeInfo {
      title: cartridge_info.title.clone(),
      licensee: cartridge_info.licensee.get_name(),
      cartridge_type: format!("{:?}", cartridge_info.cartridge_type),
      rom_size: format!("{:?}", cartridge_info.rom_size),
      ram_size: format!("{:?}", cartridge_info.ram_size),
      cgb_mode: format!("{:?}", cartridge_info.cgb_mode)

    }
  }

  pub fn press_button(&mut self, button: WebButton) {
    self.emulator.press_button(WebEmulator::convert_to_button(button));
  }

  pub fn release_button(&mut self, button: WebButton) {
    self.emulator.release_button(WebEmulator::convert_to_button(button));
  }

  pub fn cpu_info(&self) -> WebCPUInfo {
    WebEmulator::convert_to_web_cpu_info(self.emulator.cpu_info())
  }

  pub fn cartridge_info(&self) -> WebCartridgeInfo {
    WebEmulator::convert_to_web_cartridge_info(self.emulator.get_cartridge_info())
  }

  pub fn get_object(&self, object_index: u8) -> WebOAMObject {
    WebEmulator::convert_to_web_oam_object(self.emulator.get_object(object_index))
  }

  pub fn set_tile_atlas_rendering_enabled(&mut self, enabled: bool) {
    self.emulator.set_tile_atlas_rendering_enabled(enabled);
  }

  pub fn set_object_atlas_rendering_enabled(&mut self, enabled: bool) {
    self.emulator.set_object_atlas_rendering_enabled(enabled);
  }

  pub fn is_paused(&self) -> bool {
    self.emulator.is_paused()
  }

  pub fn set_paused(&mut self, paused: bool) {
    self.emulator.set_paused(paused);
  }

  pub fn run_for_nanos(&mut self, nanos: u64) {
    self.emulator.run_for_nanos(nanos);
  }

  pub fn get_state(&self) -> Result<Vec<u8>, String> {
    self.emulator.get_state()
  }

  pub fn load_state(&mut self, buffer: &[u8]) {
    self.emulator.load_state(buffer);
  }
}