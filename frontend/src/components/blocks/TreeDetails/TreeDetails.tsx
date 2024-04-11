import { ITreeDetails } from "@/types";

import "./styles.scss";

interface IProps {
  tree: ITreeDetails;
}

export const TreeDetails = (props: IProps) => {
  const state = props.tree.state ?? "healthy";

  const format = (value: number | null) => {
    if (!value) {
      return "no data";
    }

    return `${value} m`;
  };

  return (
    <div className={`TreeDetails state-${state}`}>
      <h2>{props.tree.name}</h2>
      <div className="treeId">#{props.tree.id}</div>

      <div className="props">
        <div>Height: {format(props.tree.height)}</div>
        <div>Circumference: {format(props.tree.circumference)}</div>
        <div>Canopy: {format(props.tree.diameter)}</div>
        <div>State: {state}</div>
      </div>
    </div>
  );
};
