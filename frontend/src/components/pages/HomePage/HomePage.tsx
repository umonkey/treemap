import { useState } from "react";
import { routes } from "@/utils/routes";
import { useNavigate } from "react-router-dom";

import {
  SelectLocationDialog,
  MapControl,
  SideBar,
} from "@/components";
import { IBounds, ILatLng } from "@/types";
import { useMarkers } from "./hooks";

import "./styles.css";

export const HomePage = () => {
  const { center, markers, reload } = useMarkers();
  const [picker, setPicker] = useState<boolean>(false);

  const [newPosition, setNewPosition] = useState<ILatLng | null>(null);

  const navigate = useNavigate();

  const handleBoundsChange = (bounds: IBounds) => {
    reload(bounds);
  };

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

  return (
    <div className="HomePage">
      <MapControl
        center={center}
        markers={markers}
        onAddTree={handleAddTree}
        onBoundsChange={handleBoundsChange}
        onPick={handlePicker}
        picker={picker}
      />

      {picker && (
        <SideBar>
          <SelectLocationDialog
            position={newPosition}
            onContinue={handleContinueAddingTree}
          />
        </SideBar>
      )}
    </div>
  );
};
