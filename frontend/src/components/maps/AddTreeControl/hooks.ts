// Global imports.
import { useNavigate } from "react-router-dom";

// Project imports.
import { routes } from "@/utils";

export const useAddTreeControl = () => {
  const navigate = useNavigate();

  const handleClick = (e: React.MouseEvent<HTMLAnchorElement>) => {
    e.preventDefault();
    console.debug("Navigating to add tree page.");
    navigate(routes.add());
  };

  return {
    handleClick,
  };
};
