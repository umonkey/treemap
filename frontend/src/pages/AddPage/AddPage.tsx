// Global imports.
import { ScaleControl, ZoomControl } from "react-leaflet";

// Project imports.
import { LocateControl, LocationPicker, MapBase, SelectLocationDialog, TreeMarkers, SideBar, WithHeader, WithSidebar } from "@/components";

// Local imports.
import { useAddPage } from "./hooks";
import "./styles.scss";

export const AddPage = () => {
  const { isPhone, handleLocationPick, handleContinueAddingTree, handleCancel, mapState, newPosition } = useAddPage();

  return (
    <div className="AddPage">
      <WithHeader>
        <WithSidebar>
          <MapBase
            center={mapState.center}
            zoom={mapState.zoom}
          >
            <TreeMarkers />

            {!isPhone && (
              <ZoomControl position="bottomright" />
            )}

            <LocateControl position="bottomright" />
            <ScaleControl position="bottomleft" />

            <LocationPicker onChange={handleLocationPick} />
          </MapBase>

          <SideBar>
            <SelectLocationDialog
              position={newPosition}
              onContinue={handleContinueAddingTree}
              onCancel={handleCancel}
            />
          </SideBar>
        </WithSidebar>
      </WithHeader>
    </div>
  );
};
