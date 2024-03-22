import { useRef } from "react";

import { Marker } from "react-leaflet";
import { DragEndEvent } from "leaflet";
import { ILatLng } from "@/types";

interface IProps {
  position: ILatLng | null;
  onChange: (position: ILatLng) => void;
}

export const DraggableMarker = (props: IProps) => {
  const markerRef = useRef(null);

  const handleDragEnd = (e: DragEndEvent) => {
    const pos = {
      lat: e.target.getLatLng().lat,
      lon: e.target.getLatLng().lng,
    };

    props.onChange(pos);
  };

  return props.position && (
    <Marker
      position={[props.position.lat, props.position.lon]}
      draggable={true}
      ref={markerRef}
      eventHandlers={{
        dragend: handleDragEnd,
      }}
    />
  );
};
