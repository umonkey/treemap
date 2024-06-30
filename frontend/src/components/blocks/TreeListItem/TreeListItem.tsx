import { Link } from "react-router-dom";

import { formatDate, routes } from "@/utils";

import { IProps } from "./types";
import "./styles.scss";

export const TreeListItem = (props: IProps) => {
  const { tree } = props;

  return (
    <div className="TreeListItem">
      <div className="species">
        <Link to={routes.treeDetails(tree.id.toString())}>{tree.species}</Link>
      </div>
      <div className="meta">{formatDate(tree.added_at)} by {tree.added_by}</div>
    </div>
  );
};
