// Project imports.
import {
  EditToolbarButton,
  HomeToolbarButton,
  MoveToolbarButton,
  PhotoToolbarButton,
  ShareToolbarButton,
  Toolbar,
} from "@/components";

// Local imports.
import { IProps } from "./types";
import { useDetailsToolbar } from "./hooks";
import "./styles.scss";

export const DetailsToolbar = (props: IProps) => {
  useDetailsToolbar(props);

  return (
    <Toolbar className="DetailsToolbar">
      <HomeToolbarButton />
      <EditToolbarButton id={props.id} />
      <PhotoToolbarButton id={props.id} />
      <MoveToolbarButton id={props.id} />
      <ShareToolbarButton id={props.id} />
    </Toolbar>
  );
};
