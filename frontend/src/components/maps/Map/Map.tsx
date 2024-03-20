import { MapControl } from "../MapControl";

import { useMarkers } from "./hooks";
import { IBounds } from "../../../types";

import "./styles.css";

export const Map = () => {
  const { center, markers, reload } = useMarkers();

  const handleBoundsChange = (bounds: IBounds) => {
    reload(bounds);
  };

  return (
    <MapControl
      center={center}
      markers={markers}
      onBoundsChange={handleBoundsChange}
    />
  );
};
