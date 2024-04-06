import { useState, useEffect } from "react";
import { TokenResponse, useGoogleLogin } from "@react-oauth/google";
import axios from "axios";

import { getGoogleUser, setGoogleUser } from "@/utils/storage";

interface GoogleProfile {
  id: string;
  email: string;
  locale: string;
  name: string;
  picture: string;
  verified_email: boolean;
}

export const useGoogleAuth = () => {
  const [user, setUser] = useState<TokenResponse | null>(getGoogleUser());
  const [profile, setProfile] = useState<GoogleProfile | null>(null);

  const loginFunction = useGoogleLogin({
    onSuccess: (response) => {
      console.debug("Logged in with Google:", response);

      setUser(response);
      setGoogleUser(response);
    },

    onError: (error) => {
      console.error("Error logging in with Google:", error);
    },
  });

  const login = () => {
    loginFunction();
  };

  useEffect(() => {
    if (user) {
      axios.get("https://www.googleapis.com/oauth2/v1/userinfo", {
        headers: {
          Authorization: `Bearer ${user.access_token}`,
          Accept: "application/json",
        },
      }).then((res) => {
        setProfile(res.data);
        console.debug("Got profile:", res.data);
      }).catch((error) => {
        console.error("Error getting profile:", error);
      });
    }
  }, [user]);

  return {
    profile,
    login,
  };
};
