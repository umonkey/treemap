// Global imports.
import { useMemo, useState, useEffect } from "react";

// Project imports.
import { ILatLng } from "@/types";

// Local imports.
import { IProps } from "./types";

const DIFF = 0.0001;

export const useRowSelector = (props: IProps) => {
  const [center1, setCenter1] = useState<ILatLng>({
    lat: props.center.lat,
    lon: props.center.lon - DIFF,
  });

  const [center2, setCenter2] = useState<ILatLng>({
    lat: props.center.lat,
    lon: props.center.lon + DIFF,
  });

  const path: [number, number][] = useMemo(() => {
    return [
      [center1.lat, center1.lon],
      [center2.lat, center2.lon],
    ];
  }, [center1, center2]);

  useEffect(() => {
    props.onChange(center1, center2);
  }, [center1, center2, props]);

  useEffect(() => {
    setCenter1({
      lat: props.center.lat,
      lon: props.center.lon - DIFF,
    });

    setCenter2({
      lat: props.center.lat,
      lon: props.center.lon + DIFF,
    });
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
    path,
  };
};
