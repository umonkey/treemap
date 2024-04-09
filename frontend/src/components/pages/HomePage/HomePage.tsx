import { useEffect, useState } from "react";

import {
  AddTreeDialog,
  MapControl,
  SideBar,
} from "@/components";
import { IBounds, ILatLng, ITreeInfo, SideBarMode } from "@/types";
import { useMarkers } from "./hooks";

import "./styles.css";

export const HomePage = () => {
  const { center, markers, reload } = useMarkers();
  const [sideBarMode, setSideBarMode] = useState<SideBarMode>(SideBarMode.DEFAULT);
  const [picker, setPicker] = useState<boolean>(false);
  const [bounds, setBounds] = useState<IBounds | null>(null);

  const [newPosition, setNewPosition] = useState<ILatLng | null>(null);

  const handleBoundsChange = (bounds: IBounds) => {
    setBounds(bounds);
    reload(bounds);
  };

  const handleAddTree = () => {
    setSideBarMode((prev) => prev === SideBarMode.DEFAULT ? SideBarMode.ADD_TREE : SideBarMode.DEFAULT);
  };

  const handlePicker = (position: ILatLng) => {
    setNewPosition(position);
  };

  const handleTreeCreated = (tree: ITreeInfo) => {
    console.debug(`New tree ${tree.id} created, reloading data.`, bounds);
    setSideBarMode(SideBarMode.DEFAULT);
    setPicker(false);

    bounds && reload(bounds);
  };

  const handleCancelAddingTree = () => {
    setSideBarMode(SideBarMode.DEFAULT);
    setPicker(false);
    setNewPosition(null);
  };

  useEffect(() => {
    setPicker(sideBarMode === SideBarMode.ADD_TREE);
  }, [sideBarMode]);

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

      {sideBarMode === SideBarMode.ADD_TREE && (
        <SideBar>
          <AddTreeDialog
            center={newPosition}
            onSuccess={handleTreeCreated}
            onCancel={handleCancelAddingTree}
          />
        </SideBar>
      )}
    </div>
  );
};
