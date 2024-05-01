// Global imports.
import { useState } from "react";
import { useNavigate } from "react-router-dom";

// Project imports.
import { useDeviceType, useMapState } from "@/hooks";
import { ILatLng } from "@/types";
import { routes } from "@/utils";

export const useAddPage = () => {
  const [newPosition, setNewPosition] = useState<ILatLng | null>(null);
  const { mapState } = useMapState();
  const { isPhone, isDesktop } = useDeviceType();

  const navigate = useNavigate();

  const handleLocationPick = (position: ILatLng) => {
    setNewPosition(position);
  };

  const handleContinueAddingTree = () => {
    newPosition && navigate(routes.addContinue(newPosition));
  };

  const handleCancel = () => {
    navigate(routes.home());
  };

  return {
    handleCancel,
    handleContinueAddingTree,
    handleLocationPick,
    isDesktop,
    isPhone,
    mapState,
    newPosition,
  };
};
