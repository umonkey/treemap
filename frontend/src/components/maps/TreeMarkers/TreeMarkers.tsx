/**
 * A component that loads markers from the API and displays them.
 */

import { Markers, MapEventHandler } from "@/components";
import { useMarkers } from "./hooks";
import { IBounds } from "@/types";

export const TreeMarkers = () => {
  const { markers, reload } = useMarkers();

  const handleBoundsChange = (bounds: IBounds) => {
    reload(bounds);
  };

  return (
    <>
      <MapEventHandler
        onBoundsChange={handleBoundsChange}
      />

      <Markers markers={markers} />
    </>
  );
};
