// Project imports.
import {
  MapControl,
  TreeMarkers,
  WithHeader,
  WithSidebar,
} from "@/components";

// Local imports.
import { useHomePage } from "./hooks";
import "./styles.css";

export const HomePage = () => {
  const { mapState } = useHomePage();

  return (
    <div className="HomePage">
      <WithHeader>
        <WithSidebar>
          <MapControl
            center={mapState.center}
            zoom={mapState.zoom}
          >
            <TreeMarkers />
          </MapControl>
        </WithSidebar>
      </WithHeader>
    </div>
  );
};
