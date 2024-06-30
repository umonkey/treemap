import { useUserInfo } from "@/hooks";

import { IProps } from "./types";

export const useTreeListItem = (props: IProps) => {
  const { user } = useUserInfo(props.tree.added_by);

  return {
    tree: props.tree,
    user,
  };
};
