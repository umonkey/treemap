import { createControlComponent } from '@react-leaflet/core';
import * as L from 'leaflet';
import 'leaflet.locatecontrol';
import 'leaflet.locatecontrol/dist/L.Control.Locate.css';

interface P extends L.ControlOptions {}

function createLocateInstance(props: P) {
  return new L.Control.Locate(props);
}

export const LocateControl = createControlComponent(createLocateInstance);
