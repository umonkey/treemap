import { MapControl } from "../MapControl";

import { useMarkers } from "./hooks";
import { IBounds } from "../../../types";

import "./styles.css";

interface IProps {
  onAddTree: () => void;
}

export const Map = (props: IProps) => {
  const { center, markers, reload } = useMarkers();

  const handleBoundsChange = (bounds: IBounds) => {
    reload(bounds);
  };

  const handleAddTree = () => {
    console.debug("Showing add tree form...");
    props.onAddTree();
  };

  return (
    <MapControl
      center={center}
      markers={markers}
      onAddTree={handleAddTree}
      onBoundsChange={handleBoundsChange}
      picker={false}
    />
  );
};
