// Global imports.
import { ScaleControl, ZoomControl } from "react-leaflet";
import "leaflet/dist/leaflet.css";

// Project imports.
import { ILatLng } from "@/types";
import { AddTreeControl, LocateControl, MapBase } from "@/components";

interface IProps {
  center: ILatLng;
  zoom: number;
  children?: React.ReactNode | React.ReactNode[];
}

export const MapControl = (props: IProps) => {
  return (
    <MapBase center={props.center} zoom={props.zoom}>
      <ZoomControl position="bottomright" />

      <LocateControl position="bottomright" />
      <ScaleControl position="bottomleft" />

      <AddTreeControl position="bottomright" />

      {props.children}
    </MapBase>
  );
};
