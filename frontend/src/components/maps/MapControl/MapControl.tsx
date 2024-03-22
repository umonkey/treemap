import { useState } from "react";
import { ZoomControl } from "react-leaflet";

import { IBounds, ILatLng, ITreeInfo } from "@/types";
import { AddTreeControl, DraggableMarker, LocateControl, MapBase, MapEventHandler, Markers } from "@/components";

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
  // This is the point that the user picked.  Used to display
  // a movable pointer when props.picker is true.
  const [point, setPoint] = useState<ILatLng | null>(null);

  const handleBoundsChange = (bounds: IBounds) => {
    props.onBoundsChange && props.onBoundsChange(bounds);
  };

  const handleMarkerDragged = (position: ILatLng) => {
    console.debug(`Draggable marker moved to ${position.lat}, ${position.lon}`);
    setPoint(position);
    props.onPick && props.onPick(position);
  };

  return (
    <MapBase center={props.center}>
      <MapEventHandler
        onBoundsChange={handleBoundsChange}
      />

      { !props.picker && props.markers && <Markers markers={props.markers} /> }
      { props.picker && <DraggableMarker position={point} onChange={handleMarkerDragged} /> }

      <ZoomControl position="bottomright" />
      <LocateControl position="bottomright" />

      {props.onAddTree && (
        <AddTreeControl position="bottomright" onClick={props.onAddTree} />
      )}
    </MapBase>
  );
};
