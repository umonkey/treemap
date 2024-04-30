// Global imports.
import mitt, { Emitter } from "mitt";

// Project imports.
import { ILatLng, IMarkerClickEvent } from "@/types";

type MainBusEvents = {
  before_search: void;
  tree_clicked: IMarkerClickEvent;
  pan_to: ILatLng;
};

type BusType = Emitter<MainBusEvents>;

export const mainBus: BusType = mitt<MainBusEvents>();
