// Global imports.
import { useNavigate } from "react-router-dom";

// Project imports.
import { routes } from "@/utils";

export const useTreePreviewButtons = (id: string) => {
  const navigate = useNavigate();

  const handleDetailsClick = () => {
    navigate(routes.treeDetails(id));
  };

  const handleEditClick = () => {
    navigate(routes.editTree(id));
  };

  const handleMoveClick = () => {
    navigate(routes.moveTree(id));
  };

  return {
    handleDetailsClick,
    handleEditClick,
    handleMoveClick,
  };
};
