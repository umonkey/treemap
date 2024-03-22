import { DEFAULT_API_ROOT } from "@/utils/config";

export const getApiRoot = () => {
  // Have unprefixed requests in Storybook, for simpler mocking.
  if (import.meta.env.STORYBOOK === "true") {
    return "";
  }

  const root = import.meta.env.VITE_API_ROOT || DEFAULT_API_ROOT;
  console.debug(`API Root is ${root}`);
  return root;
};
