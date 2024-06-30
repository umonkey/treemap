// Global imports.
import { create } from "zustand";

// Project imports.
import { ITreeInfo, ITreeStats, ITreeInfoMap, IUserInfo, ILoginInfo } from "@/types";
import { getLoginInfo, setLoginInfo, getMapLayer, setMapLayer } from "@/utils";
import { treeMapService } from "@/services/api";

interface IStore {
  trees: ITreeInfoMap;
  addTrees: (trees: ITreeInfo[]) => void;
  resetTrees: () => void;

  loginInfo: ILoginInfo | null;
  setLoginInfo: (value: ILoginInfo | null) => void;

  uploadProgress: number;
  setUploadProgress: (value: number) => void;

  mapLayer: string;
  setMapLayer: (value: string) => void;

  stats: ITreeStats;
  setStats: (value: ITreeStats) => void;

  // Cached user map.
  users: Map<string, IUserInfo>;
  addUsers: (value: IUserInfo[]) => void;
}

export const useStore = create<IStore>((set) => ({
  trees: {},

  /**
   * Add more trees.  Call this when you receive updates from the API.
   */
  addTrees: (trees: ITreeInfo[]) => {
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

  loginInfo: (() => {
    const info = getLoginInfo();

    if (info !== null) {
      treeMapService.setToken(info.token);
    }

    return info;
  })(),

  setLoginInfo: (value: ILoginInfo | null) => {
    setLoginInfo(value);

    set(() => {
      return { loginInfo: value };
    });
  },

  uploadProgress: 0.0,

  setUploadProgress: (value: number) => set({
    uploadProgress: value,
  }),

  mapLayer: getMapLayer(),

  setMapLayer: (value: string) => {
    setMapLayer(value);
    set({
      mapLayer: value,
    });
  },

  stats: {
    count: 0,
  } as ITreeStats,

  setStats: (value: ITreeStats) => set({
    stats: value,
  }),

  users: new Map(),

  // Add new users to the cache.
  addUsers: (value: IUserInfo[]) => {
    set((state) => {
      const updated = new Map(state.users);

      value.forEach((user) => {
        updated.set(user.id, user);
      });

      return { users: updated };
    });
  },
}));
