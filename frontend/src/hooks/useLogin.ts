// Global imports.
import { useGoogleLogin } from "@react-oauth/google";

// Project imports.
import { treeMapService } from "@/services/api";
import { useLoginInfo } from "@/hooks";

interface IProps {
  onError: () => void;
  onSuccess: () => void;
}

export const useLogin = (props: IProps) => {
  const { setLoginInfo } = useLoginInfo();

  const login = useGoogleLogin({
    onSuccess: async (response) => {
      console.debug("Google auth successful, received access token.");

      const token = response.access_token;

      try {
        const res = await treeMapService.loginGoogle(token);
        console.info("Logged in with Google.");

        setLoginInfo(res);

        props.onSuccess();
      } catch (e) {
        console.error("Error logging in with Google:", e);
        props.onError();
      }
    },

    onError: (error) => {
      console.error("Error logging in with Google:", error);
      setLoginInfo(null);
      props.onError();
    },
  });

  // The main function has a weird signature.
  const loginFunction = () => {
    login();
  };

  const logoutFunction = () => {
    setLoginInfo(null);
  };

  return {
    loginFunction,
    logoutFunction,
  };
};
