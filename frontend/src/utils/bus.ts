import mitt, { Emitter } from "mitt";
import { ILatLng } from "@/types";

type MainBusEvents = {
  centerMap: ILatLng;
};

type BusType = Emitter<MainBusEvents>;

export const mainBus: BusType = mitt<MainBusEvents>();
