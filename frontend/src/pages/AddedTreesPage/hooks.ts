import { useEffect, useState } from "react";

import { ITreeInfo } from "@/types";

import { IProps } from "./types";
import { treeMapService } from "@/services/api";

export const useAddedTreesPage = (props: IProps) => {
  const [ trees, setTrees ] = useState<ITreeInfo[]>([]);
  const [ error, setError ] = useState<string | null>(null);
  const [ loading, setLoading ] = useState<boolean>(true);

  useEffect(() => {
    (async () => {
      try {
        setLoading(true);
        const res = await treeMapService.getAddedTrees(props.count, props.skip);
        console.debug(`Found ${res.length} trees.`);
        setTrees(res);
      } catch (e) {
        console.debug(`Error loading recently added trees: ${e}`);
        setError("Error loading recently added trees, please try again later.");
      } finally {
        setLoading(false);
      }
    })();
  }, [props.count, props.skip]);

  return {
    error,
    loading,
    trees,
  };
}
