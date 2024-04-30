// Global imports.
import { create } from "zustand";

// Project imports.
import { IMarkerClickEvent, ITreeInfo, ITreeInfoMap } from "@/types";

interface IStore {
  trees: ITreeInfoMap;
  addTrees: (trees: ITreeInfo[]) => void;
  resetTrees: () => void;

  showTree: IMarkerClickEvent | null;
  setShowTree: (id: IMarkerClickEvent | null) => void;
}

export const useStore = create<IStore>((set) => ({
  trees: {},

  /**
   * Add more trees.  Call this when you receive updates from the API.
   */
  addTrees: (trees: ITreeInfo[]) => {
    console.debug(`Adding ${trees.length} trees to the store.`);

    set((state) => {
      const updated = { ...state.trees };

      trees.forEach((tree) => {
        updated[tree.id] = tree;
      });

      return { trees: updated };
    });
  },

  /**
   * Remove all known trees.  Use this after the search query is updated.
   */
  resetTrees: () => {
    console.debug("Resetting the trees store.");
    set({ trees: {} });
  },

  showTree: null,

  setShowTree: (tree: IMarkerClickEvent | null) => {
    set({ showTree: tree });
  },
}));
