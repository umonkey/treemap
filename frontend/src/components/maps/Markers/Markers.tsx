import { Link } from "react-router-dom";
import { Marker, Popup } from "react-leaflet";
import MarkerClusterGroup from "react-leaflet-cluster";

import { GreenCircleIcon } from "@/components";
import { ITreeInfo } from "@/types";
import { routes } from "@/utils/routes";

interface IProps {
  markers: ITreeInfo[];
}

export const Markers = (props: IProps) => {
  const markers = props.markers?.map((marker: ITreeInfo, index: number) => (
    <Marker key={index} position={[marker.lat, marker.lon]} icon={GreenCircleIcon}>
      <Popup><Link to={routes.treeDetails(marker.id.toString())}>{marker.name}</Link></Popup>
    </Marker>
  ));

  return (
    <MarkerClusterGroup maxClusterRadius={10}>{markers}</MarkerClusterGroup>
  );
};
