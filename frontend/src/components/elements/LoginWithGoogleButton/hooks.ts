import { useGoogleLogin } from "@react-oauth/google";

import { useUserInfo } from "@/utils/userinfo";
import { treeMapService } from "@/services/api";

interface IProps {
  onSuccess: () => void;
  onError: () => void;
}

export const useGoogleAuth = (props: IProps) => {
  const { setUserInfo } = useUserInfo();

  const loginFunction = useGoogleLogin({
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

  return {
    login: () => { loginFunction() },
  };
};
