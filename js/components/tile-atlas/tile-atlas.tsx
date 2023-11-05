import React, {useEffect} from "react";
import {WebEmulator} from "../../../pkg/rustboy_web";

import "./tile-atlas.scss"

export interface TileAtlasProps {
  emulator: WebEmulator | undefined
}

export const TileAtlas = ({ emulator }: TileAtlasProps) => {
  const enableTileAtlasRendering = () => {
    console.log('Enabling tile atlas rendering')
    emulator?.set_tile_atlas_rendering_enabled(true);

    return () => {
      console.log('Disabling tile atlas rendering')

      emulator?.set_tile_atlas_rendering_enabled(false)
    }
  }

  useEffect(enableTileAtlasRendering, [])
  useEffect(enableTileAtlasRendering, [emulator])

  return <canvas id="tile-atlas-canvas" width={ 256 } height={ 192 }></canvas>
}