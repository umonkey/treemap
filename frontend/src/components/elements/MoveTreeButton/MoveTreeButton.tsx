import { Button } from "@mui/material";
import { useNavigate } from "react-router-dom";

import { routes } from "@/utils/routes";

interface IProps {
  id: string;
}

export const MoveTreeButton = (props: IProps) => {
  const navigate = useNavigate();

  const handleClick = () => {
    navigate(routes.moveTree(props.id));
  };

  return (
    <Button color="secondary" onClick={handleClick}>Move tree</Button>
  );
};
