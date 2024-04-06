import { TokenResponse } from "@react-oauth/google";
import { GOOGLE_TOKEN_STORAGE_KEY } from "./config";

export const getGoogleUser = (): TokenResponse | null => {
  try {
    const user = localStorage.getItem(GOOGLE_TOKEN_STORAGE_KEY);

    if (user === null) {
      console.debug("No Google user found in storage.");
      return null;
    }

    console.debug("Google user info read from storage.");

    return JSON.parse(user);
  } catch (e) {
    console.error("Error reading Google user from storage:", e);
    return null;
  }
};

export const setGoogleUser = (user: TokenResponse) => {
  try {
    localStorage.setItem(GOOGLE_TOKEN_STORAGE_KEY, JSON.stringify(user));
    console.debug("Google user info saved to storage.");
  } catch (e) {
    console.error("Error storing Google user in storage:", e);
  }
};
