import { useCallback, useState, useEffect } from "react";
import { IUserInfo } from "@/types";
import { USER_INFO_KEY } from "./config";

const readStoredValue = (): IUserInfo | null => {
  try {
    const stored = localStorage.getItem(USER_INFO_KEY);

    if (stored) {
      console.debug("User info loaded from storage.");
      return JSON.parse(stored);
    }
  } catch (e) {
    console.error("Error reading user info from local storage.", e);
  }

  return null;
};

export const useUserInfo = () => {
  const [userInfo, setUserInfo] = useState<IUserInfo | null>(readStoredValue());

  const handleChange = useCallback((event: StorageEvent) => {
    if (event.key === USER_INFO_KEY) {
      console.debug("User info change detected.");
      setUserInfo(JSON.parse(event.newValue || "null"));
    }
  }, []);

  // Listen for changes.
  useEffect(() => {
    window.addEventListener("storage", handleChange);

    return () => {
      window.removeEventListener("storage", handleChange);
    }
  }, [handleChange]);

  // Modify the stored value.
  const setter = (value: IUserInfo | null) => {
    try {
      if (value) {
        localStorage.setItem(USER_INFO_KEY, JSON.stringify(value));
        console.debug("User info updated.");
      } else {
        localStorage.removeItem(USER_INFO_KEY);
        console.debug("User info removed.");
      }
    } catch (e) {
      console.error("Error updating user info.", e);
    }
  };

  return {
    userInfo,
    setUserInfo: setter,
  };
};
