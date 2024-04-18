/**
 * Simple image picker component.
 *
 * Note that it allows text/plain.  Without that, Chrome on Android
 * will not allow to take a picture with the camera, only select
 * from the gallery.  (Firefox works fine.)
 */

import { useNavigate } from "react-router-dom";
import { Button } from "@mui/material";

import { routes } from "@/utils/routes";

interface IProps {
  id: string;
}

export const AddCommentButton = (props: IProps) => {
  const navigate = useNavigate();

  const handleClick = () => {
    navigate(routes.addComment(props.id));
  };

  return (
    <Button variant="contained" onClick={handleClick}>Add a comment</Button>
  );
};
