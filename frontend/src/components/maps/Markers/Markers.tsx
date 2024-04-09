import { Marker, Popup } from "react-leaflet";
import MarkerClusterGroup from "react-leaflet-cluster";

import { GreenCircleIcon } from "@/components";
import { ITreeInfo } from "@/types";

interface IProps {
  markers: ITreeInfo[];
}

export const Markers = (props: IProps) => {
  const markers = props.markers?.map((marker: ITreeInfo, index: number) => (
    <Marker key={index} position={[marker.lat, marker.lon]} icon={GreenCircleIcon}>
      <Popup>{marker.name}</Popup>
    </Marker>
  ));

  return (
    <MarkerClusterGroup maxClusterRadius={10}>{markers}</MarkerClusterGroup>
  );
};
