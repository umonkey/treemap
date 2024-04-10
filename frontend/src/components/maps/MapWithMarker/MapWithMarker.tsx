/**
 * A simple map component with a single marker.
 * Not interactive.
 *
 * TODO: make this map non-draggable.
 */

import { Marker } from "react-leaflet";

import { MarkerIcon, MapBase } from "@/components";
import { ILatLng } from "@/types";

interface IProps {
  center: ILatLng;
}

export const MapWithMarker = (props: IProps) => {
  return (
    <MapBase center={props.center} zoom={18}>
      <Marker
        position={[props.center.lat, props.center.lon]}
        icon={MarkerIcon}
      />
    </MapBase>
  );
};
