import { Marker, Popup } from "react-leaflet";

import { MarkerIcon } from "@/components";
import { ITreeInfo } from "@/types";

interface IProps {
  markers: ITreeInfo[];
}

export const Markers = (props: IProps) => {
  return props.markers?.map((marker: ITreeInfo, index: number) => (
    <Marker key={index} position={[marker.lat, marker.lon]} icon={MarkerIcon}>
      <Popup>{marker.name}</Popup>
    </Marker>
  ));
};
