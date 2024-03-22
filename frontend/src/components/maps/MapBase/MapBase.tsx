/**
 * This is the very basic Leaflet map wrapper.
 *
 * Loads the styles, initializes layers, that's it.
 */

import { LayersControl, MapContainer, TileLayer } from "react-leaflet";

import { ILatLng } from "@/types";

import "leaflet/dist/leaflet.css";

interface IProps {
  center: ILatLng;
  children?: React.ReactNode | React.ReactNode[];
}

export const MapBase = (props: IProps) => {
  return (
    <MapContainer center={[props.center.lat, props.center.lon]} zoom={13} maxZoom={18} scrollWheelZoom={true} className="map" zoomControl={false}>
      <LayersControl position="topright">
        <LayersControl.BaseLayer checked name="OpenStreetMap">
          <TileLayer
            attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
            url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
          />
        </LayersControl.BaseLayer>
        <LayersControl.BaseLayer name="Google Maps">
          <TileLayer
            attribution='&copy; Google Maps'
            url="http://{s}.google.com/vt/lyrs=s,h&x={x}&y={y}&z={z}"
            subdomains={["mt0", "mt1", "mt2", "mt3"]}
          />
        </LayersControl.BaseLayer>
      </LayersControl>

      {props.children}
    </MapContainer>
  );
};
