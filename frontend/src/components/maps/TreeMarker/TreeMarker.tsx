/**
 * A single tree marker.
 *
 * Shows the right icon, popup.
 */

// Global imports.
import { Marker } from "react-leaflet";

// Project imports.
import { GreenCircleIcon, RedCircleIcon, BlackCircleIcon } from "@/components";
import { IMarkerClickEvent, ITreeInfo } from "@/types";
import { useStore } from "@/store";
import { mainBus } from "@/bus";

interface IProps {
  tree: ITreeInfo;
}

export const TreeMarker = (props: IProps) => {
  const setShowTree = useStore((state) => state.setShowTree);

  const getIcon = (state: string | null) => {
    if (state === "dead") {
      return BlackCircleIcon;
    }

    if (state === "sick") {
      return RedCircleIcon;
    }

    return GreenCircleIcon;
  };

  const handleMarkerClick = () => {
    const e = {
      id: props.tree.id,
      position: {
        lat: props.tree.lat,
        lon: props.tree.lon,
      }
    } as IMarkerClickEvent;

    setShowTree(e);

    mainBus.emit("tree_clicked", e);
  };

  return (
    <Marker
      position={[props.tree.lat, props.tree.lon]}
      icon={getIcon(props.tree.state)}
      eventHandlers={{ click: handleMarkerClick }}
    />
  );
}
