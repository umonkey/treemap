// Global imports.
import { useEffect } from "react";

// Project imports.
import { useMapState } from "@/hooks";
import { IMapView } from "@/types";
import { useStore } from "@/store";

export const useHomePage = () => {
  const { mapState, setMapState } = useMapState();

  const showTree = useStore((state) => state.showTree);
  const setShowTree = useStore((state) => state.setShowTree);

  const handleViewChange = ({ center, zoom }: IMapView) => {
    setMapState({ center, zoom });
  };

  useEffect(() => {
    setShowTree(null);
  }, [setShowTree]);

  const getSideBarMode = (): string | null => {
    if (showTree) {
      return "tree";
    }

    return null;
  };

  return {
    handleViewChange,
    mapState,
    showTree,
    sideBarMode: getSideBarMode(),
  };
};
