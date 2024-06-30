import { useEffect, useState } from "react";

import { ITreeInfo } from "@/types";
import { useStore } from "@/store";
import { routes } from "@/utils";

import { IProps } from "./types";
import { treeMapService } from "@/services/api";

export const useNewTreesPage = (props: IProps) => {
  const addUsers = useStore((state) => state.addUsers);

  const [ trees, setTrees ] = useState<ITreeInfo[]>([]);
  const [ error, setError ] = useState<string | null>(null);
  const [ loading, setLoading ] = useState<boolean>(true);
  const [ next, setNext ] = useState<string | null>(null);
  const [ prev, setPrev ] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      try {
        setLoading(true);

        const res = await treeMapService.getNewTrees(props.count + 1, props.skip);
        console.debug(`Found ${res.trees.length} trees, skip=${props.skip}.`);

        // Pre-populate the user cache.
        addUsers(res.users);

        if (props.skip > 0) {
          const prevLink = routes.newTreesPage(props.count, Math.max(0, props.skip - props.count));
          setPrev(prevLink);
          console.debug(`PREV: ${prevLink}`);
        } else {
          setPrev(null);
        }

        if (res.trees.length > props.count) {
          res.trees.pop();
          const nextLink = routes.newTreesPage(props.count, props.skip + props.count);
          setNext(nextLink);
          console.debug(`NEXT: ${nextLink}`);
        } else {
          setNext(null);
        }

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
    next,
    prev,
  };
}
