// Global imports.
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

// Project imports.
import { ITreeDetails } from "@/types";
import { treeMapService } from "@/services/api";
import { routes, formatDate } from "@/utils";
import { mainBus } from "@/bus";
import { useStore } from "@/store";

const formatStatusLine = (state: string, updatedAt: number): string => {
  return `${state.charAt(0).toUpperCase() + state.slice(1)}, checked on ${formatDate(updatedAt)}.`;
};

export const useTreeSidePane = (id: string) => {
  const [tree, setTree] = useState<ITreeDetails | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [status, setStatus] = useState<string | null>(null);

  const setShowTree = useStore((state) => state.setShowTree);

  const navigate = useNavigate();

  useEffect(() => {
    (async () => {
      try {
        setLoading(true);
        setError(null);
        const res = await treeMapService.getTreeDetails(id);

        setTree(res);
        setStatus(formatStatusLine(res.state, res.updated_at));

        mainBus.emit("pan_to", {
          lat: res.lat,
          lon: res.lon,
        });

        console.debug(`Tree ${res.id} info loaded.`);
      } catch (e) {
        console.error("Error loading tree info.", e);
        setError("Error loading tree info, please try again later.");
        setTree(null);
      } finally {
        setLoading(false);
      }
    })();
  }, [id]);

  const handleDetailsClick = () => {
    navigate(routes.treeDetails(id));
  };

  const handleEditClick = () => {
    navigate(routes.editTree(id));
  };

  const handleMoveClick = () => {
    navigate(routes.moveTree(id));
  };

  const handleCloseClick = () => {
    setShowTree(null);
  };

  return {
    tree,
    status,
    loading,
    error,
    handleDetailsClick,
    handleEditClick,
    handleMoveClick,
    handleCloseClick,
  };
};
