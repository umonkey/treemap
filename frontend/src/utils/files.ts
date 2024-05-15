import { getFileUrlPattern, getApiRoot } from "@/utils";

export const getFileURL = (id: string): string => {
  if (import.meta.env.STORYBOOK === "true") {
    return `https://placehold.co/600x400?text=${id}`;
  }

  const pattern = getFileUrlPattern();

  if (pattern !== null) {
    return pattern.replace("{id}", id);
  }

  const root = getApiRoot();
  return `${root}/v1/files/${id}.jpg`;
};
