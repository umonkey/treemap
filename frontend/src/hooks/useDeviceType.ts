import { useCallback, useState, useEffect } from "react";

const PHONE_WIDTH = 480;

const getHeight = (): number => {
  const { innerHeight } = window;
  return innerHeight;
};

const getWidth = (): number => {
  const { innerWidth } = window;
  return innerWidth;
};

const isPhoneDevice = (width: number, height: number): boolean => {
  const min = Math.min(width, height);
  return min < PHONE_WIDTH;
};

const getClassName = (width: number, height: number, hasTouch: boolean): string => {
  const parts = [];

  if (isPhoneDevice(width, height)) {
    parts.push("phone");
  } else {
    parts.push("desktop");
  }

  if (height > width) {
    parts.push("portrait");
  } else {
    parts.push("landscape");
  }

  if (hasTouch) {
    parts.push("touch");
  } else {
    parts.push("notouch");
  }

  return parts.join(" ");
};

export const useDeviceType = () => {
  const [width, setWidth] = useState(getWidth());
  const [height, setHeight] = useState(getHeight());
  const [phone, setPhone] = useState<boolean>(isPhoneDevice(width, height));
  const [portrait, setPortrait] = useState<boolean>(height > width);
  const [touch] = useState<boolean>("ontouchstart" in document.documentElement);
  const [className, setClassName] = useState<string>(getClassName(width, height, touch));

  const handleResize = useCallback(() => {
    setWidth(getWidth());
    setHeight(getHeight());
    setPhone(isPhoneDevice(getWidth(), getHeight()));
    setPortrait(getHeight() > getWidth());
    setClassName(getClassName(getWidth(), getHeight(), touch));
  }, [touch]);

  useEffect(() => {
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, [handleResize]);

  useEffect(() => {
    console.debug("CLASS NAME CHANGE", className);
    document.getElementById("root")!.className = className;
  }, [className]);

  return {
    isPhone: phone,
    isDesktop: !phone,
    isPortrait: portrait,
    haTouch: touch,
    className,
  };
};
