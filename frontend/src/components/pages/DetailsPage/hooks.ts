import { useState, useEffect } from "react";

import { ITreeDetails } from "@/types";
import { treeMapService } from "@/services/api";

export const useTreeDetails = (id: string) => {
  const [tree, setTree] = useState<ITreeDetails | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(true);

  const canShare = !!navigator.share;

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

  const handleShare = () => {
    if (!tree) {
      return;
    }

    navigator.share({
      title: tree.name,
      text: "Check out this tree on the Tree Map!",
    });
  };

  return {
    tree,
    loading,
    error,
    canShare,
    handleShare,
  };
};
