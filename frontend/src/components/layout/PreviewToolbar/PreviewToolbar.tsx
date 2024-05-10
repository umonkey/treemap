// Global imports.
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCamera } from "@fortawesome/free-solid-svg-icons";

// Project imports.
import { EditToolbarButton, InfoToolbarButton, MoveToolbarButton, ShareToolbarButton } from "@/components";

// Local imports.
import { IProps } from "./types";
import { usePreviewToolbar } from "./hooks";
import "./styles.scss";

export const PreviewToolbar = (props: IProps) => {
  usePreviewToolbar(props);

  return (
    <div className="PreviewToolbar">
      <InfoToolbarButton id={props.id} />
      <EditToolbarButton id={props.id} />

      <button>
        <div className="icon">
          <FontAwesomeIcon icon={faCamera} />
        </div>
        <div className="label">Photo</div>
      </button>

      <MoveToolbarButton id={props.id} />
      <ShareToolbarButton id={props.id} />
    </div>
  );
};
