import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faUpDownLeftRight } from "@fortawesome/free-solid-svg-icons";

import { useMoveToolbarButton } from "./hooks";
import { IProps } from "./types";

export const MoveToolbarButton = (props: IProps) => {
  const {
    handleClick,
  } = useMoveToolbarButton(props);

  return (
    <button onClick={handleClick}>
      <div className="icon">
        <FontAwesomeIcon icon={faUpDownLeftRight} />
      </div>
      <div className="label">Move</div>
    </button>
  );
};
