import { Link } from "react-router-dom";

import { ITreeDetails } from "@/types";
import { formatTreeDimensions, routes } from "@/utils";
import { formatState } from "./utils";
import { treeMapService } from "@/services/api";
import { formatDate } from "@/utils";
import "./styles.scss";

interface IProps {
  tree: ITreeDetails;
}

export const TreePopup = ({ tree }: IProps) => {
  return (
    <div className="TreePopup">
      {tree.thumbnail_id && (
        <Link to={routes.treeDetails(tree.id.toString())}>
          <img src={treeMapService.getFileURL(tree.thumbnail_id)} alt={tree.species} />
        </Link>
      )}

      <div className="inside">
        <div className="header">
          <Link className="title" to={routes.treeDetails(tree.id.toString())}>{tree.species}</Link>
        </div>

        {tree.state && (
          <div className="state">{formatState(tree.state)}</div>
        )}

        <div className="meta">{formatTreeDimensions(tree)}</div>

        {tree.updated_at && (
          <div className="timestamp">Updated on {formatDate(tree.updated_at)}</div>
        )}
      </div>
    </div>
  );
};
