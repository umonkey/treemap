// Project imports.
import { useStore } from "@/store";

export const useLoginInfo = () => {
  const loginInfo = useStore((state) => state.loginInfo);
  const setLoginInfo = useStore((state) => state.setLoginInfo);

  return {
    loginInfo,
    setLoginInfo,
  };
};
