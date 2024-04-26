/**
 * A component that loads markers from the API and displays them.
 */

import { Markers, MapEventHandler } from "@/components";
import { useMarkers } from "./hooks";

export const TreeMarkers = () => {
  const { markers, handleBoundsChange } = useMarkers();

  return (
    <>
      <MapEventHandler
        onBoundsChange={handleBoundsChange}
      />

      <Markers markers={markers} />
    </>
  );
};
