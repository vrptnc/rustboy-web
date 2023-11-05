import React, {useEffect, useState} from "react";
import {CartridgeInfo, WebEmulator} from "../../../pkg/rustboy_web";

import './cartridge-info-panel.scss'

export interface CartridgeInfoPanelProps {
  emulator: WebEmulator | undefined
}

export const CartridgeInfoPanel = ({ emulator }: CartridgeInfoPanelProps) => {
  const [info, setInfo] = useState<CartridgeInfo>()

  useEffect(() => {
    setInfo(emulator?.cartridge_info())
  }, [emulator])

  return <div className="cartridge-info">
      <div className="label">Title</div>
      <div className="value">{info?.title}</div>
      <div className="label">Licensee</div>
      <div className="value">{info?.licensee}</div>
      <div className="label">Cartridge Type</div>
      <div className="value">{info?.cartridge_type}</div>
      <div className="label">ROM Size</div>
      <div className="value">{info?.rom_size}</div>
      <div className="label">RAM Size</div>
      <div className="value">{info?.ram_size}</div>
      <div className="label">CGB Mode</div>
      <div className="value">{info?.cgb_mode}</div>
  </div>
}