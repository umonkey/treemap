import Control from "react-leaflet-custom-control";
import * as L from 'leaflet';
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faTree } from "@fortawesome/free-solid-svg-icons";

interface IProps {
  position: L.ControlPosition;
  onClick: () => void;
}

export const AddTreeControl = ({ position, onClick }: IProps) => {
  const handleClick = (e: React.MouseEvent<HTMLAnchorElement>) => {
    e.preventDefault();
    onClick();
  };

  return (
    <Control prepend position={position}>
      <div className="leaflet-bar" title="Add a tree">
        <a className="leaflet-bar-part leaflet-bar-part-single" href="#" onClick={handleClick}>
          <FontAwesomeIcon icon={faTree} />
        </a>
      </div>
    </Control>
  );
};
