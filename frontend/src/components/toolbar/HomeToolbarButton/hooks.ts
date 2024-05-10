import { useNavigate } from "react-router-dom";

import { routes } from "@/utils";

export const useHomeToolbarButton = () => {
  const navigate = useNavigate();

  const handleClick = () => {
    navigate(routes.home());
  };

  return {
    handleClick,
  };
};
