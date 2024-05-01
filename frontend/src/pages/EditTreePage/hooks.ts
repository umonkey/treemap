import { useEffect, useState } from "react";

import { treeMapService } from "@/services/api";
import { ITreeDetails } from "@/types";

export const useTreeInfo = (id: string) => {
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
        console.error(`Error reading tree info: ${e}`);
        setError("Error getting tree info, please try again later.");
      } finally {
        setLoading(false);
      }
    })();
  }, [id]);

  return {
    tree,
    error,
    loading,
  };
};
