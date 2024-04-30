/**
 * A simple map component with a single marker.
 * Not interactive.
 *
 * TODO: make this map non-draggable.
 */

import { DefaultMarker, MapBase } from "@/components";
import { ILatLng } from "@/types";
import "./styles.scss";

interface IProps {
  center: ILatLng;
  children?: React.ReactNode | React.ReactNode[];
}

export const MapWithMarker = (props: IProps) => {
  return (
    <div className="MapWithMarker">
      <MapBase center={props.center} zoom={18}>
        <DefaultMarker center={props.center} />
        {props.children}
      </MapBase>
    </div>
  );
};
