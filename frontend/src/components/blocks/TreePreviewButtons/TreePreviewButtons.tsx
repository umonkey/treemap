import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Button, ButtonGroup } from "@mui/material";
import { faPencil, faUpDownLeftRight } from "@fortawesome/free-solid-svg-icons";

import { useTreePreviewButtons } from "./hooks";

interface IProps {
  id: string;
}

export const TreePreviewButtons = (props: IProps) => {
  const { handleDetailsClick, handleEditClick, handleMoveClick } = useTreePreviewButtons(props.id);

  return (
    <div className="buttons">
      <ButtonGroup variant="contained">
        <Button onClick={handleDetailsClick}>Details</Button>
        <Button onClick={handleEditClick}>
          <FontAwesomeIcon icon={faPencil} />
        </Button>
        <Button onClick={handleMoveClick}>
          <FontAwesomeIcon icon={faUpDownLeftRight} />
        </Button>
      </ButtonGroup>
    </div>
  );
};
