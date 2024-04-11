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

      <ul>
        <li>Height: {props.tree.height || "no data"}</li>
        <li>Circumference: {props.tree.circumference || "no data"}</li>
        <li>Canopy: {props.tree.diameter || "no data"}</li>
      </ul>
    </div>
  );
};
