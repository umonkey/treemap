// Global imports.
import { useState } from "react";
import { useNavigate } from "react-router-dom";

// Project imports.
import { routes } from "@/utils/routes";
import { useMapState } from "@/hooks";
import {
  MapControl,
  MapEventHandler,
  WithHeader,
  SelectLocationDialog,
  SideBar,
  TreeMarkers,
  WithSidebar,
} from "@/components";
import { ILatLng, IMapView } from "@/types";

// Local imports.
import "./styles.css";

export const HomePage = () => {
  const [picker, setPicker] = useState<boolean>(false);
  const [newPosition, setNewPosition] = useState<ILatLng | null>(null);
  const { mapState, setMapState } = useMapState();

  const navigate = useNavigate();

  const handleAddTree = () => {
    setPicker(!picker);
  };

  const handlePicker = (position: ILatLng) => {
    setNewPosition(position);
  };

  const handleContinueAddingTree = () => {
    if (newPosition) {
      navigate(routes.addTree(newPosition));
    }
  };

  const handleCancel = () => {
    setPicker(false);
  };

  const handleViewChange = ({ center, zoom }: IMapView) => {
    setMapState({ center, zoom });
  };

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
          </MapControl>

          {picker && (
            <SideBar>
              <SelectLocationDialog
                position={newPosition}
                onContinue={handleContinueAddingTree}
                onCancel={handleCancel}
              />
            </SideBar>
          )}
        </WithSidebar>
      </WithHeader>
    </div>
  );
};
