// Global imports.
import { Button, ButtonGroup } from "@mui/material";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faLocationDot, faUpDown, faLeftRight, faCircleNotch, faCircleInfo, faX } from "@fortawesome/free-solid-svg-icons";

// Project imports.
import { ExternalTreeLinks } from "@/components";

// Local imports.
import { useTreeSidePane } from "./hooks";
import "./styles.scss";

interface IProps {
  id: string;
}

export const TreeSidePane = (props: IProps) => {
  const { tree, status, loading, error, handleDetailsClick, handleEditClick, handleMoveClick, handleCloseClick } = useTreeSidePane(props.id);

  return (
    <div className="TreeSidePane">
      {loading && !tree && <p>Loading tree details...</p>}

      {error && (
        <p className="error">{error}</p>
      )}

      {!error && tree && (
        <>
          <h2>{tree.species}</h2>
          <div className="treeId">#{tree.id}</div>

          {tree.notes && (
            <div className="notes">{tree.notes}</div>
          )}

          <ExternalTreeLinks tree={tree} />

          <div className="buttons">
            <ButtonGroup variant="contained">
              <Button onClick={handleDetailsClick}>Details</Button>
              <Button onClick={handleEditClick}>Edit</Button>
              <Button onClick={handleMoveClick}>Move</Button>
            </ButtonGroup>
          </div>

          <ul className="props">
            <li className="status"><FontAwesomeIcon icon={faCircleInfo} /> {status}</li>
            <li className="location"><FontAwesomeIcon icon={faLocationDot} /> {tree.lat.toFixed(6)}, {tree.lon.toFixed(6)}</li>
            {tree.height && (
              <li><FontAwesomeIcon icon={faUpDown} /> {tree.height} m</li>
            )}
            {tree.diameter && (
              <li><FontAwesomeIcon icon={faLeftRight} /> {tree.diameter} m</li>
            )}
            {tree.circumference && (
              <li><FontAwesomeIcon icon={faCircleNotch} /> {tree.circumference} m</li>
            )}
          </ul>
        </>
      )}

      <button className="close" onClick={handleCloseClick}><FontAwesomeIcon icon={faX} /></button>
    </div>
  );
};
