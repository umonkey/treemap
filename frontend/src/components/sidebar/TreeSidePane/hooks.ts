// Global imports.
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

// Project imports.
import { ITreeDetails } from "@/types";
import { routes, formatTreeDimensions, formatDate } from "@/utils";
import { mainBus } from "@/bus";

const formatStatusLine = (state: string, updatedAt: number): string => {
  return `${state.charAt(0).toUpperCase() + state.slice(1)}, checked on ${formatDate(updatedAt)}.`;
};

export const useTreeSidePane = (tree: ITreeDetails) => {
  const [status, setStatus] = useState<string | null>(null);
  const [dimensions, setDimensions] = useState<string | null>(null);

  const navigate = useNavigate();

  useEffect(() => {
    mainBus.emit("pan_to", {
      lat: tree.lat,
      lon: tree.lon,
    });

    setStatus(formatStatusLine(tree.state, tree.updated_at));
    setDimensions(formatTreeDimensions(tree));
  }, [tree]);

  const handleCloseClick = () => {
    navigate(routes.home());
  };

  return {
    tree,
    status,
    handleCloseClick,
    dimensions,
  };
};
