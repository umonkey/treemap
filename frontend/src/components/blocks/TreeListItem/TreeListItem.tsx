import { Link } from "react-router-dom";

import { formatDate, routes } from "@/utils";

import { IProps } from "./types";
import { useTreeListItem } from "./hooks";
import "./styles.scss";

export const TreeListItem = (props: IProps) => {
  const { tree, user } = useTreeListItem(props);

  return (
    <div className="TreeListItem">
      <div className="species">
        <Link to={routes.treeDetails(tree.id.toString())}>{tree.species}</Link>
      </div>
      <div className="meta">{formatDate(tree.added_at)} by {user?.name || "unknown user"}</div>
    </div>
  );
};
