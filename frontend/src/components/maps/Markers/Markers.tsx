import { Marker, Popup } from "react-leaflet";
import MarkerClusterGroup from "react-leaflet-cluster";

import { GreenCircleIcon, TreePopup } from "@/components";
import { ITreeInfo } from "@/types";

import "./styles.scss";

interface IProps {
  markers: ITreeInfo[];
}

export const Markers = (props: IProps) => {
  const markers = props.markers?.map((marker: ITreeInfo) => (
    <Marker key={marker.id} position={[marker.lat, marker.lon]} icon={GreenCircleIcon}>
      <Popup>
        <TreePopup tree={marker} />
      </Popup>
    </Marker>
  ));

  return (
    <MarkerClusterGroup maxClusterRadius={10}>{markers}</MarkerClusterGroup>
  );
};
