import { useEffect, useState } from "react";

import { ITreeDetails } from "@/types";
import { treeMapService } from "@/services/api";
import { useMapState } from "@/hooks";

export const usePreviewPage = (id: string) => {
  const [tree, setTree] = useState<ITreeDetails | null>(null);
  const { mapState } = useMapState();
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      try {
        setError(null);

        const res = await treeMapService.getTreeDetails(id);
        setTree(res);
      } catch (e) {
        console.error("Error loading tree.", e);
        setError("Error loading tree, please try again later.");
      }
    })();

    // Fetch tree details by id.
    // setTree(response.data);
  }, [id]);

  return {
    error,
    mapState,
    tree,
  };
};
