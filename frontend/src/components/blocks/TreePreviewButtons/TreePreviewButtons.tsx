// Global imports.
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Button, ButtonGroup } from "@mui/material";
import { faCamera, faPencil, faUpDownLeftRight } from "@fortawesome/free-solid-svg-icons";

// Project imports.
import { ImagePicker } from "@/components";

// Local imports.
import { useTreePreviewButtons } from "./hooks";
import "./styles.scss";

interface IProps {
  id: string;
}

export const TreePreviewButtons = (props: IProps) => {
  const {
    handleDetailsClick,
    handleEditClick,
    handleImageUpload,
    handleMoveClick,
  } = useTreePreviewButtons(props.id);

  return (
    <div className="TreePreviewButtons buttons">
      <ButtonGroup variant="contained">
        <Button onClick={handleDetailsClick}>Details</Button>
        <Button onClick={handleEditClick}>
          <FontAwesomeIcon icon={faPencil} />
        </Button>
        <ImagePicker onChange={handleImageUpload}>
          <FontAwesomeIcon icon={faCamera} />
        </ImagePicker>
        <Button onClick={handleMoveClick}>
          <FontAwesomeIcon icon={faUpDownLeftRight} />
        </Button>
      </ButtonGroup>
    </div>
  );
};
