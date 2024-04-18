import { ITreeDetails } from "@/types";

interface IStateMap {
  [key: string]: string;
}

const STATES = {
  healthy: "Looks good.",
  sick: "Looks sick.",
  dead: "Looks dead.",
  deformed: "Looks deformed.",
  gone: "Was removed completely.",
} as IStateMap;

export const formatMeta = (tree: ITreeDetails): string => {
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
}

export const formatState = (state: string): string => {
  return STATES[state] ?? `Looks ${state}.`;
};
