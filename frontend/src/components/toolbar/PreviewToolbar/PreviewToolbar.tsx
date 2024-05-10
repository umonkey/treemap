// Project imports.
import { EditToolbarButton, InfoToolbarButton, MoveToolbarButton, PhotoToolbarButton, ShareToolbarButton, Toolbar } from "@/components";

// Local imports.
import { IProps } from "./types";
import { usePreviewToolbar } from "./hooks";
import "./styles.scss";

export const PreviewToolbar = (props: IProps) => {
  usePreviewToolbar(props);

  return (
    <Toolbar className="PreviewToolbar">
      <InfoToolbarButton id={props.id} />
      <EditToolbarButton id={props.id} />
      <PhotoToolbarButton id={props.id} />
      <MoveToolbarButton id={props.id} />
      <ShareToolbarButton id={props.id} />
    </Toolbar>
  );
};
