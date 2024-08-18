/**
 * A single tree marker.
 *
 * Shows the right icon, popup.
 */

// Global imports.
import { Marker } from "react-leaflet";
import { useNavigate } from "react-router-dom";

// Project imports.
import { GreenCircleIcon, RedCircleIcon, BlackCircleIcon } from "@/components";
import { IMarkerClickEvent, ITreeInfo } from "@/types";
import { mainBus } from "@/bus";
import { routes } from "@/utils";

interface IProps {
  tree: ITreeInfo;
}

export const TreeMarker = (props: IProps) => {
  const navigate = useNavigate();

  const getIcon = (state: string | null) => {
    if (state === "dead" || state === "gone" || state === "stomp") {
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

    navigate(routes.treePreview(props.tree.id));

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
