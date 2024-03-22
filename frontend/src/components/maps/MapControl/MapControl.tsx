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
    setPoint(position);
  };

  const handleMapClick = (position: ILatLng) => {
      if (props.picker) {
        setPoint({ lat: position.lat, lon: position.lon });
        console.debug(`New point picked: ${position.lat}, ${position.lon}`);
      }
  };

  return (
    <MapBase center={props.center}>
      <MapEventHandler
        onClick={handleMapClick}
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
