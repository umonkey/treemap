import { useNavigate } from "react-router-dom";

import { routes } from "@/utils";

import { IProps } from "./types";

export const useInfoToolbarButton = (props: IProps) => {
  const navigate = useNavigate();

  const handleClick = () => {
    navigate(routes.treeDetails(props.id));
  };

  return {
    handleClick,
  };
};
