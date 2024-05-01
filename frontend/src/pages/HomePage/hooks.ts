// Global imports.
import { useEffect } from "react";

// Project imports.
import { useMapState } from "@/hooks";
import { useStore } from "@/store";

export const useHomePage = () => {
  const { mapState } = useMapState();

  const showTree = useStore((state) => state.showTree);
  const setShowTree = useStore((state) => state.setShowTree);

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
    mapState,
    showTree,
    sideBarMode: getSideBarMode(),
  };
};
