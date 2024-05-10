import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faHouse } from "@fortawesome/free-solid-svg-icons";

import { useHomeToolbarButton } from "./hooks";

export const HomeToolbarButton = () => {
  const {
    handleClick,
  } = useHomeToolbarButton();

  return (
    <button onClick={handleClick}>
      <div className="icon">
        <FontAwesomeIcon icon={faHouse} />
      </div>
      <div className="label">Home</div>
    </button>
  );
};
