// Global imports.
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faRulerCombined, faCircleInfo, faX } from "@fortawesome/free-solid-svg-icons";

// Project imports.
import { GalleryLoader, TreePreviewButtons, ExternalTreeLinks } from "@/components";
import { ITreeDetails } from "@/types";

// Local imports.
import { useTreeSidePane } from "./hooks";
import "./styles.scss";

interface IProps {
  tree: ITreeDetails;
}

export const TreeSidePane = (props: IProps) => {
  const { tree, dimensions, status, handleCloseClick } = useTreeSidePane(props.tree);

  return (
    <div className="TreeSidePane">
      <h2>{tree.species}</h2>

      {tree.notes && (
        <div className="notes">{tree.notes}</div>
      )}

      <ExternalTreeLinks tree={tree} />

      <TreePreviewButtons id={tree.id} />

      <ul className="props">
        <li className="status"><FontAwesomeIcon icon={faCircleInfo} /> {status}</li>
        {dimensions && (
          <li><FontAwesomeIcon icon={faRulerCombined} /> {dimensions}</li>
        )}
      </ul>

      <GalleryLoader id={tree.id} />

      <button className="close" onClick={handleCloseClick}><FontAwesomeIcon icon={faX} /></button>
    </div>
  );
};
