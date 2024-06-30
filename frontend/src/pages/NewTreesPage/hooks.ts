import { useEffect, useState } from "react";

import { ITreeInfo } from "@/types";
import { useStore } from "@/store";

import { IProps } from "./types";
import { treeMapService } from "@/services/api";

export const useNewTreesPage = (props: IProps) => {
  const addUsers = useStore((state) => state.addUsers);

  const [ trees, setTrees ] = useState<ITreeInfo[]>([]);
  const [ error, setError ] = useState<string | null>(null);
  const [ loading, setLoading ] = useState<boolean>(true);

  useEffect(() => {
    (async () => {
      try {
        setLoading(true);
        const res = await treeMapService.getNewTrees(props.count, props.skip);

        // Pre-populate the user cache.
        addUsers(res.users);

        console.debug(`Found ${res.trees.length} trees.`);
        setTrees(res.trees);
      } catch (e) {
        console.debug(`Error loading recently added trees: ${e}`);
        setError("Error loading recently added trees, please try again later.");
      } finally {
        setLoading(false);
      }
    })();
  }, [props.count, props.skip, addUsers]);

  return {
    error,
    loading,
    trees,
  };
}
