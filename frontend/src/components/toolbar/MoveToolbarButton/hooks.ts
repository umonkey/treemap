import { useNavigate } from "react-router-dom";

import { routes } from "@/utils";

import { IProps } from "./types";

export const useMoveToolbarButton = (props: IProps) => {
  const navigate = useNavigate();

  const handleClick = () => {
    navigate(routes.editTree(props.id));
  };

  return {
    handleClick,
  };
};
