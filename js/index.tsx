import React from 'react';
import ReactDOM from 'react-dom';
import { App } from './components/app/app';
import { default as init} from '../pkg/rustboy_web'
import rustboyWasm from '../pkg/rustboy_web_bg.wasm'

declare global {
  interface Window {
    webkitAudioContext: AudioContext
  }
}

init(rustboyWasm).then(() => {
  ReactDOM.render(<App />, document.body);
})
