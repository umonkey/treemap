import { ITreeDetails } from "@/types";

import "./styles.scss";

interface IProps {
  tree: ITreeDetails;
}

export const TreeDetails = (props: IProps) => {
  return (
    <div className="TreeDetails">
      <h2>{props.tree.name}</h2>
      <div className="treeId">#{props.tree.id}</div>
      <p>This tree was added recently, pending verification.</p>
    </div>
  );
};
