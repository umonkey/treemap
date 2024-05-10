import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faShareNodes } from "@fortawesome/free-solid-svg-icons";

import { useShareToolbarButton } from "./hooks";
import { IProps } from "./types";

export const ShareToolbarButton = (props: IProps) => {
  const {
    handleClick,
  } = useShareToolbarButton(props);

  return (
    <button onClick={handleClick}>
      <div className="icon">
        <FontAwesomeIcon icon={faShareNodes} />
      </div>
      <div className="label">Share</div>
    </button>
  );
};
