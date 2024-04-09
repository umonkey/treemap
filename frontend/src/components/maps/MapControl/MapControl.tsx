import { ZoomControl } from "react-leaflet";

import { IBounds, ILatLng, ITreeInfo } from "@/types";
import { AddTreeControl, LocateControl, LocationPicker, MapBase, MapEventHandler, Markers } from "@/components";
import { useMobileDevice } from "./hooks";

import "leaflet/dist/leaflet.css";

interface IProps {
  center: ILatLng;
  picker: boolean;
  onAddTree?: () => void;
  onBoundsChange?: (bounds: IBounds) => void;
  onPick?: (position: ILatLng) => void;
  markers?: ITreeInfo[];
}

export const MapControl = (props: IProps) => {
  const isMobile = useMobileDevice();

  const handleBoundsChange = (bounds: IBounds) => {
    props.onBoundsChange && props.onBoundsChange(bounds);
  };

  const handleLocationPick = (position: ILatLng) => {
    props.onPick && props.onPick(position);
  };

  return (
    <MapBase center={props.center}>
      <MapEventHandler
        onBoundsChange={handleBoundsChange}
      />

      { props.markers && <Markers markers={props.markers} /> }

      {props.picker && (
        <LocationPicker onChange={handleLocationPick} />
      )}

      {!isMobile && (
        <ZoomControl position="bottomright" />
      )}

      <LocateControl position="bottomright" />

      {props.onAddTree && (
        <AddTreeControl position="bottomright" onClick={props.onAddTree} />
      )}
    </MapBase>
  );
};
