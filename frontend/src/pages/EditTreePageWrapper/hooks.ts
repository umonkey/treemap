import { useParams, useNavigate } from "react-router-dom";

import { routes } from "@/utils";

export const useEditTreePageWrapper = () => {
  const { id } = useParams();

  const navigate = useNavigate();

  if (!id) {
    throw Error("Tree id not set.");
  }

  const handleSuccess = () => {
    navigate(routes.treeDetails(id));
  };

  const handleCancel = () => {
    navigate(routes.treeDetails(id));
  };

  return {
    id,
    handleSuccess,
    handleCancel,
  };
}
