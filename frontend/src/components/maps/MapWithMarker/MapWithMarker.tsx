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
  children?: React.ReactNode | React.ReactNode[];
}

export const MapWithMarker = (props: IProps) => {
  return (
    <div className="MapWithMarker">
      <MapBase center={props.center} zoom={18}>
        <Marker position={[props.center.lat, props.center.lon]} icon={MarkerIcon} />
        {props.children}
      </MapBase>
    </div>
  );
};
