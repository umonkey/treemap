// Global imports.
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

// Project imports.
import { ITreeDetails } from "@/types";
import { treeMapService } from "@/services/api";
import { routes, formatTreeDimensions, formatDate } from "@/utils";
import { mainBus } from "@/bus";

const formatStatusLine = (state: string, updatedAt: number): string => {
  return `${state.charAt(0).toUpperCase() + state.slice(1)}, checked on ${formatDate(updatedAt)}.`;
};

export const useTreeSidePane = (id: string) => {
  const [tree, setTree] = useState<ITreeDetails | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [status, setStatus] = useState<string | null>(null);
  const [dimensions, setDimensions] = useState<string | null>(null);

  const navigate = useNavigate();

  useEffect(() => {
    (async () => {
      try {
        setLoading(true);
        setError(null);

        const res = await treeMapService.getTreeDetails(id);

        setTree(res);
        setStatus(formatStatusLine(res.state, res.updated_at));
        setDimensions(formatTreeDimensions(res));

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

  const handleCloseClick = () => {
    navigate(routes.home());
  };

  return {
    tree,
    status,
    loading,
    error,
    handleCloseClick,
    dimensions,
  };
};
