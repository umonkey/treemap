// Global imports.
import { Marker } from "react-leaflet";

// Local imports.
import { useDraggableMarker } from "./hooks";
import { IProps } from "./types";

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
