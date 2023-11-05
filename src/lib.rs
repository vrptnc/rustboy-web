pub mod web_emulator;
pub mod web_audio_driver;
pub mod canvas_renderer;
pub mod bindings;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

