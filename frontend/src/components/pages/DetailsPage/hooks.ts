import { useState, useEffect } from "react";

import { ITreeDetails } from "@/types";
import { treeMapService } from "@/services/api";

export const useTreeDetails = (id: number) => {
  const [tree, setTree] = useState<ITreeDetails | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    (async () => {
      try {
        setLoading(true);
        const res = await treeMapService.getTreeDetails(id);
        setTree(res);
      } catch (e) {
        setError("Error loading tree details, please try again later.");
      } finally {
        setLoading(false);
      }
    })();
  }, [id]);

  return {
    tree,
    loading,
    error,
  };
};
