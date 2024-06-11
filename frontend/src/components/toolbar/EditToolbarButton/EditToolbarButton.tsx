import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faPencil } from "@fortawesome/free-solid-svg-icons";

import { locale } from "@/locale";
import { useEditToolbarButton } from "./hooks";
import { IProps } from "./types";

export const EditToolbarButton = (props: IProps) => {
  const {
    handleClick,
  } = useEditToolbarButton(props);

  return (
    <button onClick={handleClick}>
      <div className="icon">
        <FontAwesomeIcon icon={faPencil} />
      </div>
      <div className="label">{locale.editButton()}</div>
    </button>
  );
};
