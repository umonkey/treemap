import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCircleInfo } from "@fortawesome/free-solid-svg-icons";

import { IProps } from "./types";
import { useInfoToolbarButton } from "./hooks";

export const InfoToolbarButton = (props: IProps) => {
  const {
    handleClick,
  } = useInfoToolbarButton(props);

  return (
    <button onClick={handleClick}>
      <div className="icon">
        <FontAwesomeIcon icon={faCircleInfo} />
      </div>
      <div className="label">Details</div>
    </button>
  );
};
