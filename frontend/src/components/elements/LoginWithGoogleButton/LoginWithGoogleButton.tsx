/**
 * A button to log in with Google.
 *
 * To know when a user logged in successfully, see `src/hooks/useUserInfo.ts`.
 */

import { Button } from "@mui/material";
import { locale } from "@/locale";
import { useGoogleAuth } from "./hooks";

interface IProps {
  disabled?: boolean;
  onSuccess: () => void;
  onError: () => void;
}

export const LoginWithGoogleButton = (props: IProps) => {
  const { loginFunction } = useGoogleAuth({
    onSuccess: props.onSuccess,
    onError: props.onError,
  });

  return (
    <Button variant="contained" color="success" disabled={props.disabled} onClick={loginFunction}>{locale.logInWithGoogle()}</Button>
  );
};
