import { useLogin, useLoginInfo } from "@/hooks";

export const useProfileHeaderButton = () => {
  const { loginInfo } = useLoginInfo();

  const handleSuccess = () => {
    // ignore for now
  };

  const handleError = () => {
    // ignore for now
  };

  const { loginFunction, logoutFunction } = useLogin({
    onSuccess: handleSuccess,
    onError: handleError,
  });

  return {
    isLoggedIn: !!loginInfo,
    loginFunction,
    logoutFunction,
  };
};
