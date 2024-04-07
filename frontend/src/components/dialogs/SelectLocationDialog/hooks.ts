import { useState, useEffect } from "react";
import { TokenResponse, useGoogleLogin } from "@react-oauth/google";

import { useUserInfo } from "@/utils/userinfo";
import { treeMapService } from "@/services/api";

export const useGoogleAuth = () => {
  const [token, setToken] = useState<TokenResponse | null>(null);
  const { userInfo, setUserInfo } = useUserInfo();

  const loginFunction = useGoogleLogin({
    onSuccess: (response) => {
      console.debug("Received a token from Google.");
      setToken(response);
    },

    onError: (error) => {
      console.error("Error logging in with Google:", error);
      setUserInfo(null);
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
        } catch (e) {
          console.error("Error logging in with Google:", e);
        }
      }
    })();
  }, [token, setUserInfo]);

  return {
    userInfo,
    login: () => { loginFunction() },
  };
};
