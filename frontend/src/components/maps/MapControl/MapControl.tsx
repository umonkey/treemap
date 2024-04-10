import { ZoomControl } from "react-leaflet";

import { ILatLng } from "@/types";
import { AddTreeControl, LocateControl, LocationPicker, MapBase } from "@/components";
import { useMobileDevice } from "./hooks";

import "leaflet/dist/leaflet.css";

interface IProps {
  center: ILatLng;
  picker: boolean;
  onAddTree?: () => void;
  onPick?: (position: ILatLng) => void;
  children?: React.ReactNode | React.ReactNode[];
}

export const MapControl = (props: IProps) => {
  const isMobile = useMobileDevice();

  const handleLocationPick = (position: ILatLng) => {
    props.onPick && props.onPick(position);
  };

  return (
    <MapBase center={props.center}>
      {!isMobile && (
        <ZoomControl position="bottomright" />
      )}

      <LocateControl position="bottomright" />

      {props.onAddTree && (
        <AddTreeControl position="bottomright" onClick={props.onAddTree} />
      )}

      {props.children}

      {props.picker && (
        <LocationPicker onChange={handleLocationPick} />
      )}
    </MapBase>
  );
};
