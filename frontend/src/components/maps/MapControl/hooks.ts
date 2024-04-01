import { useEffect, useState } from "react";

export const useMobileDevice = () => {
  const [isMobile, setIsMobile] = useState(false);

  useEffect(() => {
    const hasTouchScreen = "ontouchstart" in document.documentElement;
    setIsMobile(hasTouchScreen);
  }, []);

  return isMobile;
};
