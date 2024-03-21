import { DEFAULT_API_ROOT } from "@/config";

export const getApiRoot = () => {
  return import.meta.env.REACT_APP_API_ROOT || DEFAULT_API_ROOT;
};
