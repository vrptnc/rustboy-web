import React, {Fragment, MouseEvent, useEffect, useState} from "react";
import {OAMObject, WebEmulator} from "../../../pkg/rustboy_web";

import "./object-atlas.scss"

export interface ObjectAtlasProps {
  emulator: WebEmulator | undefined
}

export const ObjectAtlas = ({ emulator }: ObjectAtlasProps) => {

  const [objectInfoIndex, setObjectInfoIndex] = useState<number>()
  const [selectedObject, setSelectedObject] = useState<OAMObject>()

  const enableObjectAtlasRendering = () => {
    emulator?.set_object_atlas_rendering_enabled(true)
    return () => emulator?.set_object_atlas_rendering_enabled(false)
  }

  useEffect(enableObjectAtlasRendering, [])
  useEffect(enableObjectAtlasRendering, [emulator])

  useEffect(() => {
    if (objectInfoIndex != null && emulator != null) {
      const object = emulator.get_object(objectInfoIndex)
      setSelectedObject(object)
    } else {
      setSelectedObject(undefined)
    }

  }, [objectInfoIndex])

  const onMouseMoveInObjectCanvas = (event: MouseEvent) => {
    const infoElement = event.currentTarget as Element
    const infoElementStyle = window.getComputedStyle(infoElement)
    const infoElementBoundingRect = infoElement.getBoundingClientRect()
    const infoElementWidth = infoElementBoundingRect.width
    const infoElementHeight = infoElementBoundingRect.height
    const x = Math.round(event.clientX - infoElementBoundingRect.left - parseFloat(infoElementStyle.borderLeftWidth ?? '0'))
    const y = Math.round(event.clientY - infoElementBoundingRect.top - parseFloat(infoElementStyle.borderTopWidth ?? '0'))
    if (x >= 0 && x < infoElementWidth && y >= 0 && y <= infoElementHeight) {
      const objectIndex = Math.floor(2 * y  / infoElementHeight) * 16 + Math.floor(20 * x / infoElementWidth)
      setObjectInfoIndex(objectIndex)
    } else {
      setObjectInfoIndex(undefined)
    }
  }

  const onMouseLeaveObjectCanvas = () => {
    setObjectInfoIndex(undefined)
  }

  const ObjectInfoContainer = () =>
    <div className={ 'object-info-container' + (selectedObject ? '' : ' hidden') }>
      <div className="label">X</div>
      <div className="value">{ selectedObject?.lcd_x }</div>
      <div className="label">Y</div>
      <div className="value">{ selectedObject?.lcd_y }</div>
      <div className="label">Tile Index</div>
      <div className="value"> { selectedObject?.tile_index }</div>
      <div className="label">Attributes</div>
      <div className="value">{ `0x${ selectedObject?.attributes.toString(16) }` }</div>
    </div>

  return <div className="object-atlas">
    <canvas id="object-atlas-canvas" onMouseMove={ onMouseMoveInObjectCanvas } onMouseLeave={ onMouseLeaveObjectCanvas }
            width={ 160 } height={ 32 }></canvas>
    <ObjectInfoContainer/>
  </div>
}