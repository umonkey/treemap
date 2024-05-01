import { ScaleControl, ZoomControl } from "react-leaflet";

import { ILatLng } from "@/types";
import { AddTreeControl, LocateControl, MapBase } from "@/components";
import { useMobileDevice } from "./hooks";

import "leaflet/dist/leaflet.css";

interface IProps {
  center: ILatLng;
  zoom: number;
  children?: React.ReactNode | React.ReactNode[];
}

export const MapControl = (props: IProps) => {
  const isMobile = useMobileDevice();

  return (
    <MapBase center={props.center} zoom={props.zoom}>
      {!isMobile && (
        <ZoomControl position="bottomright" />
      )}

      <LocateControl position="bottomright" />
      <ScaleControl position="bottomleft" />

      <AddTreeControl position="bottomright" />

      {props.children}
    </MapBase>
  );
};
