// Global imports.
import { useState, useEffect } from "react";

// Project imports.
import { mainBus } from "@/bus";

export const useProgressBar = () => {
  const [progress, setProgress] = useState<number>(0);

  useEffect(() => {
    const handler = (value: number) => {
      setProgress(value);
    };

    mainBus.on("upload_progress", handler);
    return () => mainBus.off("upload_progress", handler);
  });

  return {
    progress,
  };
};
