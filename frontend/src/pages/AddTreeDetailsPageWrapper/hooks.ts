// Global imports.
import { useSearchParams } from "react-router-dom";

const getFloatParam = (params: URLSearchParams, key: string) => {
  const value = params.get(key);
  return value ? parseFloat(value) : null;
};

export const useAddTreeDetailsPageWrapper = () => {
  const [params] = useSearchParams();

  const lat = getFloatParam(params, "lat");
  const lon = getFloatParam(params, "lon");

  if (!lat || !lon) {
    throw Error("Missing lat or lon in URL.");
  }

  return {
    lat,
    lon,
  };
};
