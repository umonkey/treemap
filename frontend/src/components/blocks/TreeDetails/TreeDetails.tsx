import { ITreeDetails } from "@/types";

import "./styles.scss";

interface IProps {
  tree: ITreeDetails;
}

export const TreeDetails = (props: IProps) => {
  const format = (value: number | null) => {
    if (value === null) {
      return "no data";
    }

    return `${value} m`;
  };

  return (
    <div className="TreeDetails">
      <h2>{props.tree.name}</h2>
      <div className="treeId">#{props.tree.id}</div>

      <ul className="props">
        <li>Height: {format(props.tree.height)}</li>
        <li>Circumference: {format(props.tree.circumference)}</li>
        <li>Canopy: {format(props.tree.diameter)}</li>
      </ul>
    </div>
  );
};
