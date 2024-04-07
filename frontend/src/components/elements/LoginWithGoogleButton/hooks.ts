import { useState, useEffect } from "react";
import { TokenResponse, useGoogleLogin } from "@react-oauth/google";

import { useUserInfo } from "@/utils/userinfo";
import { treeMapService } from "@/services/api";

interface IProps {
  onSuccess: () => void;
  onError: () => void;
}

export const useGoogleAuth = (props: IProps) => {
  const [token, setToken] = useState<TokenResponse | null>(null);
  const { setUserInfo } = useUserInfo();

  const loginFunction = useGoogleLogin({
    onSuccess: (response) => {
      console.debug("Received a token from Google.");
      setToken(response);
    },

    onError: (error) => {
      console.error("Error logging in with Google:", error);
      setUserInfo(null);
      props.onError();
    },
  });

  // When a Google token is received, exchange it for user info.
  useEffect(() => {
    (async () => {
      if (token) {
        console.debug("Access token received from Google. Logging in.");

        try {
          const res = await treeMapService.loginGoogle(token.access_token);

          setUserInfo(res);
          console.info("Logged in with Google.");

          props.onSuccess();
        } catch (e) {
          console.error("Error logging in with Google:", e);
          props.onError();
        }
      }
    })();
  }, [token, setUserInfo, props]);

  return {
    login: () => { loginFunction() },
  };
};
