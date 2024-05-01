// Global imports.
import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

// Project imports.
import { routes } from "@/utils/routes";
import { useMapState } from "@/hooks";
import { ILatLng, IMapView } from "@/types";
import { useStore } from "@/store";

export const useHomePage = () => {
  const [picker, setPicker] = useState<boolean>(false);
  const [newPosition, setNewPosition] = useState<ILatLng | null>(null);
  const { mapState, setMapState } = useMapState();

  const showTree = useStore((state) => state.showTree);
  const setShowTree = useStore((state) => state.setShowTree);

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

  useEffect(() => {
    setShowTree(null);
  }, [setShowTree]);

  const getSideBarMode = (): string | null => {
    if (picker) {
      return "picker";
    }

    if (showTree) {
      return "tree";
    }

    return null;
  };

  return {
    handleAddTree,
    handleCancel,
    handleContinueAddingTree,
    handlePicker,
    handleViewChange,
    mapState,
    newPosition,
    picker,
    showTree,
    sideBarMode: getSideBarMode(),
  };
};
