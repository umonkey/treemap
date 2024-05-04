// Project imports.
import { useStore } from "@/store";

export const useUserInfo = () => {
  const userInfo = useStore((state) => state.userInfo);
  const setUserInfo = useStore((state) => state.setUserInfo);

  return {
    userInfo,
    setUserInfo,
  };
};
