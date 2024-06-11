import { ExternalTreeLinks } from "@/components";
import { ITreeDetails } from "@/types";
import { getFileURL } from "@/utils";
import { locale } from "@/locale";

import "./styles.scss";

interface IProps {
  tree: ITreeDetails;
}

export const TreeDetails = (props: IProps) => {
  const state = props.tree.state ?? "healthy";

  const format = (value: number | null) => {
    if (!value) {
      return locale.noData();
    }

    return locale.sizeMeters(value);
  };

  const image_id = props.tree.thumbnail_id;

  return (
    <div className={`TreeDetails state-${state}`}>
      {image_id && (
        <img className="thumbnail" src={getFileURL(image_id)} alt="preview" />
      )}

      <div className="inside">
        <h2>{locale.speciesTitle(props.tree.species)}</h2>

        <div className="treeId">#{props.tree.id}</div>

        {props.tree.notes && (
          <h3>{props.tree.notes}</h3>
        )}

        <div className="props">
          <div>{locale.height()}: {format(props.tree.height)}</div>
          <div>{locale.circumference()}: {format(props.tree.circumference)}</div>
          <div>{locale.canopy()}: {format(props.tree.diameter)}</div>
          <div>{locale.state()}: {locale.stateValue(state)}</div>
        </div>

        <ExternalTreeLinks tree={props.tree} />
      </div>
    </div>
  );
};
