import { ITreeInfo } from "@/types";

export const formatTreeDimensions = (tree: ITreeInfo): string => {
  const parts = [];

  if (tree.height) {
    parts.push(`H= ${tree.height.toFixed(1)} m`);
  }

  if (tree.circumference) {
    parts.push(`C= ${tree.circumference.toFixed(2)} m`);
  }

  if (tree.diameter) {
    parts.push(`D= ${tree.diameter.toFixed(1)} m`);
  }

  return parts.join(", ");
};
