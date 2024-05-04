import { useLogin, useUserInfo } from "@/hooks";

export const useProfileHeaderButton = () => {
  const { userInfo } = useUserInfo();

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
    isLoggedIn: !!userInfo,
    loginFunction,
    logoutFunction,
  };
};
