// Project imports.
import { useLogin } from "@/hooks";

interface IProps {
  onSuccess: () => void;
  onError: () => void;
}

export const useGoogleAuth = (props: IProps) => {
  const { loginFunction } = useLogin({
    onSuccess: props.onSuccess,
    onError: props.onError,
  });

  return {
    loginFunction,
  };
};
