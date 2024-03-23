/**
 * This is the very basic Leaflet map wrapper.
 *
 * Loads the styles, initializes layers, that's it.
 */

import { MapContainer } from "react-leaflet";
import { LayerSelector } from "@/components";
import { ILatLng } from "@/types";

import "leaflet/dist/leaflet.css";

interface IProps {
  center: ILatLng;
  children?: React.ReactNode | React.ReactNode[];
}

export const MapBase = (props: IProps) => {
  return (
    <MapContainer center={[props.center.lat, props.center.lon]} zoom={13} maxZoom={18} scrollWheelZoom={true} className="map" zoomControl={false}>
      <LayerSelector />

      {props.children}
    </MapContainer>
  );
};
