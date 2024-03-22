import { useRef } from "react";

import { Marker, useMapEvents } from "react-leaflet";
import { DragEndEvent } from "leaflet";
import { ILatLng } from "@/types";

interface IProps {
  position: ILatLng | null;
  onChange: (position: ILatLng) => void;
}

export const DraggableMarker = (props: IProps) => {
  const markerRef = useRef(null);

  const reportMove = (lat: number, lon: number) => {
    props.onChange({
      lat,
      lon,
    });
  }

  const handleDragEnd = (e: DragEndEvent) => {
    reportMove(e.target.getLatLng().lat, e.target.getLatLng().lng);
  };

  useMapEvents({
    click: (e) => {
      reportMove(e.latlng.lat, e.latlng.lng);

      const marker = markerRef.current;

      if (marker !== null) {
        // @ts-expect-error TS2339
        marker.setLatLng(e.latlng);
      }
    }
  });

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
