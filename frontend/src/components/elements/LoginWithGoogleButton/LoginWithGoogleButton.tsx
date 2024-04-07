/**
 * A button to log in with Google.
 *
 * To know when a user logged in successfully, see `src/utils/userinfo.ts`.
 */

import { Button } from "@mui/material";
import { useGoogleAuth } from "./hooks";

interface IProps {
  disabled?: boolean;
  onSuccess: () => void;
  onError: () => void;
}

export const LoginWithGoogleButton = (props: IProps) => {
  const { login } = useGoogleAuth({
    onSuccess: props.onSuccess,
    onError: props.onError,
  });

  const handleClick = () => {
    login();
  };

  return (
    <Button variant="contained" color="success" disabled={props.disabled} onClick={handleClick}>Log In with Google</Button>
  );
};
