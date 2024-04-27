import { useLogin, useUserInfo } from "@/hooks";

export const useProfileHeaderButton = () => {
  const { userInfo } = useUserInfo();

  const handleSuccess = () => {
    window.location.reload();
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
