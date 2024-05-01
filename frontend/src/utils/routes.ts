import { ILatLng } from "@/types";

export const routes = {
  add: () => "/add",
  addComment: (tree_id: string) => `/tree/${tree_id}/comment`,
  addContinue: (position: ILatLng) => `/add/continue?lat=${position.lat}&lon=${position.lon}`,
  editTree: (id: string) => `/tree/${id}/edit`,
  home: () => "/",
  moveTree: (id: string) => `/tree/${id}/move`,
  search: (query: string) => `/?search=${query}`,
  treeDetails: (id: string) => `/tree/${id}`,
  treePreview: (id: string) => `/tree/${id}/preview`,
};
