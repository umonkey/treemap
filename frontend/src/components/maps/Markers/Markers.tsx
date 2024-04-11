import { Link } from "react-router-dom";
import { Marker, Popup } from "react-leaflet";
import MarkerClusterGroup from "react-leaflet-cluster";

import { GreenCircleIcon } from "@/components";
import { ITreeInfo } from "@/types";
import { routes } from "@/utils/routes";

import "./styles.scss";

interface IProps {
  markers: ITreeInfo[];
}

export const Markers = (props: IProps) => {
  const markers = props.markers?.map((marker: ITreeInfo) => (
    <Marker key={marker.id} position={[marker.lat, marker.lon]} icon={GreenCircleIcon}>
      <Popup>
        <div className="TreePopup">
          <Link className="title" to={routes.treeDetails(marker.id.toString())}>{marker.name}</Link>
        </div>
      </Popup>
    </Marker>
  ));

  return (
    <MarkerClusterGroup maxClusterRadius={10}>{markers}</MarkerClusterGroup>
  );
};
