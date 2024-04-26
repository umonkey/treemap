import { ILatLng } from "@/types";

export const routes = {
  home: () => "/",
  addComment: (tree_id: string) => `/tree/${tree_id}/comment`,
  addTree: (position: ILatLng) => `/add?lat=${position.lat}&lon=${position.lon}`,
  treeDetails: (id: string) => `/tree/${id}`,
  editTree: (id: string) => `/tree/${id}/edit`,
  moveTree: (id: string) => `/tree/${id}/move`,
  search: (query: string) => `/?search=${query}`,
};
