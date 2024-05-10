// Global imports.
import { useNavigate } from "react-router-dom";

// Project imports.
import { routes } from "@/utils";
import { useFileUploader } from "@/hooks";

export const useTreePreviewButtons = (id: string) => {
  const navigate = useNavigate();
  const { uploadFiles } = useFileUploader();

  const handleDetailsClick = () => {
    navigate(routes.treeDetails(id));
  };

  const handleEditClick = () => {
    navigate(routes.editTree(id));
  };

  const handleMoveClick = () => {
    navigate(routes.moveTree(id));
  };

  const handleImageUpload = (files: FileList) => {
    uploadFiles(id, files);
  };

  return {
    handleDetailsClick,
    handleEditClick,
    handleImageUpload,
    handleMoveClick,
  };
};
