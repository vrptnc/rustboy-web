use js_sys::Array;
use rustboy_core::audio::{AudioDriver, Channel, CustomWaveOptions, NoiseOptions, PulseOptions, StereoChannel};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{AudioContext, AudioNode, AudioParamMap, AudioWorkletNode, AudioWorkletNodeOptions, BiquadFilterType, GainNode};

#[wasm_bindgen]
pub struct WebAudioDriver {
  ch1_node: AudioWorkletNode,
  ch2_node: AudioWorkletNode,
  ch3_node: AudioWorkletNode,
  ch4_node: AudioWorkletNode,
  ch1_gain_node: GainNode,
  ch2_gain_node: GainNode,
  ch3_gain_node: GainNode,
  ch4_gain_node: GainNode,
}

#[wasm_bindgen]
impl WebAudioDriver {
  pub fn new(context: AudioContext) -> WebAudioDriver {
    let destination = context.destination();
    let mut worklet_node_options = AudioWorkletNodeOptions::new();
    let output_channel_counts = Array::new();
    output_channel_counts.push(&JsValue::from(2));
    worklet_node_options.number_of_inputs(0);
    worklet_node_options.number_of_outputs(1);
    worklet_node_options.output_channel_count(&output_channel_counts);
    let ch1_node = AudioWorkletNode::new_with_options(&context, "pwm-processor", &worklet_node_options).unwrap();
    let ch2_node = AudioWorkletNode::new_with_options(&context, "pwm-processor", &worklet_node_options).unwrap();
    let ch3_node = AudioWorkletNode::new_with_options(&context, "waveform-processor", &worklet_node_options).unwrap();
    let ch4_node = AudioWorkletNode::new_with_options(&context, "white-noise-processor", &worklet_node_options).unwrap();

    let mixer_node = context.create_gain().unwrap();
    let high_pass_filter_node = context.create_biquad_filter().unwrap();
    high_pass_filter_node.set_type(BiquadFilterType::Highpass);
    high_pass_filter_node.frequency().set_value(20.0f32);

    let connect_ch_node = |node: &AudioNode| -> GainNode {
      let gain_node = context.create_gain().unwrap();
      gain_node.gain().set_value(0.25);
      node.connect_with_audio_node(&gain_node).unwrap();
      gain_node.connect_with_audio_node(&mixer_node).unwrap();
      gain_node
    };

    let ch1_gain_node = connect_ch_node(&ch1_node);
    let ch2_gain_node = connect_ch_node(&ch2_node);
    let ch3_gain_node = connect_ch_node(&ch3_node);
    let ch4_gain_node = connect_ch_node(&ch4_node);

    mixer_node.connect_with_audio_node(&high_pass_filter_node).unwrap();
    high_pass_filter_node.connect_with_audio_node(&destination).unwrap();

    WebAudioDriver {
      ch1_node,
      ch2_node,
      ch3_node,
      ch4_node,
      ch1_gain_node,
      ch2_gain_node,
      ch3_gain_node,
      ch4_gain_node
    }
  }

  fn get_parameters(&self, channel: Channel) -> AudioParamMap {
    match channel {
      Channel::CH1 => self.ch1_node.parameters().unwrap(),
      Channel::CH2 => self.ch2_node.parameters().unwrap(),
      Channel::CH3 => self.ch3_node.parameters().unwrap(),
      Channel::CH4 => self.ch4_node.parameters().unwrap(),
    }
  }
}

impl AudioDriver for WebAudioDriver {
  fn play_pulse(&mut self, channel: Channel, pulse_options: PulseOptions) {
    let parameters: AudioParamMap = match channel {
      Channel::CH1 => self.ch1_node.parameters().unwrap(),
      Channel::CH2 => self.ch2_node.parameters().unwrap(),
      _ => panic!("Can't play pulse on channel other than 1 or 2")
    };
    let frequency_param = parameters.get("frequency").unwrap();
    frequency_param.set_value(pulse_options.frequency);
    let duty_cycle_param = parameters.get("dutyCycle").unwrap();
    duty_cycle_param.set_value(pulse_options.duty_cycle);
    let trigger_param = parameters.get("trigger").unwrap();
    trigger_param.set_value(1.0);
  }

  fn play_custom_wave(&mut self, channel: Channel, wave_options: CustomWaveOptions) {
    let parameters = self.get_parameters(channel);
    (0..8usize).for_each(|index| {
      let offset = 2 * index;
      let value = (wave_options.data[offset] as u32 +
        ((wave_options.data[offset + 1] as u32) << 8)) as f32;
      parameters.get(format!("data{}", index).as_str()).unwrap().set_value(value);
    });
    let trigger_param = parameters.get("trigger").unwrap();
    trigger_param.set_value(1.0);
  }

  fn play_noise(&mut self, channel: Channel, noise_options: NoiseOptions) {
    let parameters = self.get_parameters(channel);
    let frequency_param = parameters.get("frequency").unwrap();
    frequency_param.set_value(44100.0f32.min(noise_options.frequency));
    let width_param = parameters.get("width").unwrap();
    width_param.set_value(if noise_options.short { 1.0 } else { 0.0 });
    let trigger_param = parameters.get("trigger").unwrap();
    trigger_param.set_value(1.0);
  }

  fn stop(&mut self, channel: Channel) {
    let parameters = self.get_parameters(channel);
    let trigger_param = parameters.get("trigger").unwrap();
    trigger_param.set_value(0.0);
  }

  fn set_gain(&mut self, channel: Channel, gain: f32) {
    let parameters = self.get_parameters(channel);
    let gain_param = parameters.get("gain").unwrap();
    gain_param.set_value(gain);
  }

  fn set_stereo_gain(&mut self, channel: Channel, stereo_channel: StereoChannel, gain: f32) {
    let parameters = self.get_parameters(channel);
    let stereo_gain_param = match stereo_channel {
      StereoChannel::Left => parameters.get("leftChannelGain").unwrap(),
      StereoChannel::Right => parameters.get("rightChannelGain").unwrap()
    };
    stereo_gain_param.set_value(gain);
  }

  fn set_frequency(&mut self, channel: Channel, frequency: f32) {
    let parameters = self.get_parameters(channel);
    let frequency_param = parameters.get("frequency").unwrap();
    frequency_param.set_value(frequency);
  }

  fn mute_all(&mut self) {
    self.ch1_gain_node.gain().set_value(0.0);
    self.ch2_gain_node.gain().set_value(0.0);
    self.ch3_gain_node.gain().set_value(0.0);
    self.ch4_gain_node.gain().set_value(0.0);
  }

  fn unmute_all(&mut self) {
    self.ch1_gain_node.gain().set_value(0.25);
    self.ch2_gain_node.gain().set_value(0.25);
    self.ch3_gain_node.gain().set_value(0.25);
    self.ch4_gain_node.gain().set_value(0.25);
  }

  fn set_master_volume(&mut self, _value: u8) {
    todo!()
  }
}