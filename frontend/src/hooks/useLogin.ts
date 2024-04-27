// Global imports.
import { useGoogleLogin } from "@react-oauth/google";

// Project imports.
import { treeMapService } from "@/services/api";
import { useUserInfo } from "@/hooks";

interface IProps {
  onError: () => void;
  onSuccess: () => void;
}

export const useLogin = (props: IProps) => {
  const { setUserInfo } = useUserInfo();

  const login = useGoogleLogin({
    onSuccess: async (response) => {
      console.debug("Google auth successful, received access token.");

      const token = response.access_token;

      try {
        const res = await treeMapService.loginGoogle(token);
        console.info("Logged in with Google.");

        setUserInfo(res);

        props.onSuccess();
      } catch (e) {
        console.error("Error logging in with Google:", e);
        props.onError();
      }
    },

    onError: (error) => {
      console.error("Error logging in with Google:", error);
      setUserInfo(null);
      props.onError();
    },
  });

  // The main function has a weird signature.
  const loginFunction = () => {
    login();
  };

  const logoutFunction = () => {
    setUserInfo(null);
    window.location.reload();
  };

  return {
    loginFunction,
    logoutFunction,
  };
};
