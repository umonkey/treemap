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
    parts.push(`H= ${tree.height} m`);
  }

  if (tree.circumference) {
    parts.push(`C= ${tree.circumference} m`);
  }

  if (tree.diameter) {
    parts.push(`D= ${tree.diameter} m`);
  }

  return parts.join(", ");
}

export const formatDate = (tree: ITreeDetails): string => {
  const date = new Date(tree.updated_at * 1000);
  return `${date.getDate()}.${date.getMonth() + 1}.${date.getFullYear()}`;
}

export const formatState = (state: string): string => {
  return STATES[state] ?? `Looks ${state}.`;
};