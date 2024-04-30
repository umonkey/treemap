// Global imports.
import { Marker } from "react-leaflet";

// Project imports.
import { MarkerIcon } from "@/components";
import { ILatLng } from "@/types";

interface IProps {
  center: ILatLng;
}

export const DefaultMarker = (props: IProps) => {
  return (
    <Marker position={[props.center.lat, props.center.lon]} icon={MarkerIcon} />
  );
};
