import { useState, useEffect } from "react";

const PHONE_WIDTH = 480;

const getWidth = (): number => {
  const { innerWidth } = window;
  return innerWidth;
};

export const useDeviceType = () => {
  const [width, setWidth] = useState(getWidth());

  const isPhone = width < PHONE_WIDTH;

  const handleResize = () => {
    setWidth(getWidth());
  };

  useEffect(() => {
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  return {
    isPhone,
    isDesktop: !isPhone,
  };
};
