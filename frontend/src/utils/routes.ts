import { ILatLng } from "@/types";

export const routes = {
  home: () => "/",
  addTree: (position: ILatLng) => `/add?lat=${position.lat}&lon=${position.lon}`,
  treeDetails: (id: string) => `/tree/${id}`,
};
