import { Link } from "react-router-dom";

import { ITreeDetails } from "@/types";
import { routes } from "@/utils/routes";
import { formatMeta, formatDate, formatState } from "./utils";
import "./styles.scss";

interface IProps {
  tree: ITreeDetails;
}

export const TreePopup = ({ tree }: IProps) => {
  return (
    <div className="TreePopup">
      <div className="header">
        <Link className="title" to={routes.treeDetails(tree.id.toString())}>{tree.name}</Link>
      </div>

      {tree.state && (
        <div className="state">{formatState(tree.state)}</div>
      )}

      <div className="meta">{formatMeta(tree)}</div>

      {tree.updated_at && (
        <div className="timestamp">Updated on {formatDate(tree)}</div>
      )}
    </div>
  );
};
