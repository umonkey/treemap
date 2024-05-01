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

export const formatState = (state: string): string => {
  return STATES[state] ?? `Looks ${state}.`;
};
