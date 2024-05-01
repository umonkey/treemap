// Project imports.
import {
  DefaultMarker,
  MapControl,
  MapEventHandler,
  SelectLocationDialog,
  SideBar,
  TreeMarkers,
  TreeSidePane,
  WithHeader,
  WithSidebar,
} from "@/components";

// Local imports.
import { useHomePage } from "./hooks";
import "./styles.css";

export const HomePage = () => {
  const {
    handleAddTree,
    handleCancel,
    handleContinueAddingTree,
    handlePicker,
    handleViewChange,
    mapState,
    newPosition,
    picker,
    sideBarMode,
    showTree,
  } = useHomePage();

  return (
    <div className="HomePage">
      <WithHeader>
        <WithSidebar>
          <MapControl
            center={mapState.center}
            zoom={mapState.zoom}
            onAddTree={handleAddTree}
            onPick={handlePicker}
            picker={picker}
          >
            <MapEventHandler
              onViewChange={handleViewChange}
            />

            <TreeMarkers />

            {showTree && (
              <DefaultMarker center={showTree.position} />
            )}
          </MapControl>

          {sideBarMode === "picker" && (
            <SideBar>
              <SelectLocationDialog
                position={newPosition}
                onContinue={handleContinueAddingTree}
                onCancel={handleCancel}
              />
            </SideBar>
          )}

          {sideBarMode === "tree" && showTree && (
            <SideBar>
              <TreeSidePane id={showTree.id} />
            </SideBar>
          )}
        </WithSidebar>
      </WithHeader>
    </div>
  );
};
