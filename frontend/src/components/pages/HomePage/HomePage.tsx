import { useState } from "react";
import { routes } from "@/utils/routes";
import { useNavigate } from "react-router-dom";

import {
  SelectLocationDialog,
  MapControl,
  SideBar,
  TreeMarkers,
} from "@/components";

import { ILatLng } from "@/types";

import "./styles.css";

const CENTER = {
  lat: 40.181389,
  lon: 44.514444,
} as ILatLng;

export const HomePage = () => {
  const [picker, setPicker] = useState<boolean>(false);
  const [newPosition, setNewPosition] = useState<ILatLng | null>(null);

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

  return (
    <div className="HomePage">
      <MapControl
        center={CENTER}
        onAddTree={handleAddTree}
        onPick={handlePicker}
        picker={picker}
      >
        <TreeMarkers />
      </MapControl>

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
