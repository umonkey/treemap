// Global imports.
import { useNavigate } from "react-router-dom";

// Project imports.
import { routes } from "@/utils";

// Local imports.
import { IProps } from "./types";

export const useDetailsToolbar = (props: IProps) => {
  const navigate = useNavigate();

  const handleEditClick = () => {
    navigate(routes.editTree(props.id));
  };

  const handleHomeClick = () => {
    navigate(routes.home());
  };

  const handleMoveClick = () => {
    navigate(routes.moveTree(props.id));
  };

  return {
    handleEditClick,
    handleHomeClick,
    handleMoveClick,
  };
};
