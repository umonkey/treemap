// Global imports.
import { Marker } from "react-leaflet";

// Project imports.
import { ILatLng } from "@/types";

// Local imports.
import { useDraggableMarker } from "./hooks";

interface IProps {
  center: ILatLng;
  onChange: (center: ILatLng) => void;
}

export const DraggableMarker = (props: IProps) => {
  const { center, eventHandlers, ref } = useDraggableMarker(props);

  return (
    <Marker
      draggable={true}
      position={[center.lat, center.lon]}
      ref={ref}
      eventHandlers={eventHandlers}
    />
  );
};
