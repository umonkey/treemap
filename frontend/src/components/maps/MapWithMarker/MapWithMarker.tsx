/**
 * A simple map component with a single marker.
 * Not interactive.
 *
 * TODO: make this map non-draggable.
 */

import { DefaultMarker, MapBase } from "@/components";
import { ILatLng } from "@/types";

// Local imports.
import { useMapWithMarker } from "./hooks";
import "./styles.scss";

interface IProps {
  center: ILatLng;
  children?: React.ReactNode | React.ReactNode[];
}

export const MapWithMarker = (props: IProps) => {
const { zoom } = useMapWithMarker();

  return (
    <div className="MapWithMarker">
      <MapBase center={props.center} zoom={zoom} fullscreenControl>
        <DefaultMarker center={props.center} />
        {props.children}
      </MapBase>
    </div>
  );
};
