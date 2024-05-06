// Global imports.
import { useState, useEffect } from "react";

// Project imports.
import { ILatLng } from "@/types";

interface IProps {
  center: ILatLng;
}

export const useRowSelector = (props: IProps) => {
  const [center1, setCenter1] = useState<ILatLng>(props.center);
  const [center2, setCenter2] = useState<ILatLng>(props.center);

  useEffect(() => {
    setCenter1(props.center);
    setCenter2(props.center);
  }, [props.center]);

  const handleChange1 = (center: ILatLng) => {
    setCenter1(center);
  };

  const handleChange2 = (center: ILatLng) => {
    setCenter2(center);
  };

  return {
    center1,
    center2,
    handleChange1,
    handleChange2,
  };
};
