import { useState } from "react";

import { ILatLng } from "@/types";

export const useAddPage = () => {
  const [points, setPoints] = useState<ILatLng[]>([]);

  const handlePointsChange = (points: ILatLng[]) => {
    setPoints(points);
  };

  return {
    handlePointsChange,
    points,
  };
};
