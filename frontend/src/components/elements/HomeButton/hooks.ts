import { useNavigate } from "react-router-dom";

import { routes } from "@/utils/routes";

export const useHomeButton = () => {
  const navigate = useNavigate();

  const handleClick = () => {
    console.debug("nagivate to home");
    navigate(routes.home());
  };

  return {
    handleClick,
  };
};
