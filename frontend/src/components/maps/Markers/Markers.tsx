import { MarkerCluster, TreeMarker } from "@/components";
import { ITreeInfo } from "@/types";

interface IProps {
  markers: ITreeInfo[];
}

export const Markers = (props: IProps) => {
  const markers = props.markers?.map((marker: ITreeInfo) => (
    <TreeMarker key={marker.id} tree={marker} />
  ));

  return (
    <MarkerCluster>{markers}</MarkerCluster>
  );
};
