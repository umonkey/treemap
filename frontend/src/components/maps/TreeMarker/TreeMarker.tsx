/**
 * A single tree marker.
 *
 * Shows the right icon, popup.
 */

import { Marker, Popup } from "react-leaflet";

import { GreenCircleIcon, RedCircleIcon, BlackCircleIcon, TreePopup } from "@/components";
import { ITreeInfo } from "@/types";

interface IProps {
  tree: ITreeInfo;
}

export const TreeMarker = (props: IProps) => {
  const getIcon = (state: string | null) => {
    if (state === "dead") {
      return BlackCircleIcon;
    }

    if (state === "sick") {
      return RedCircleIcon;
    }

    return GreenCircleIcon;
  };

  return (
    <Marker position={[props.tree.lat, props.tree.lon]} icon={getIcon(props.tree.state)}>
      <Popup autoPan={false}>
        <TreePopup tree={props.tree} />
      </Popup>
    </Marker>
  );
}
