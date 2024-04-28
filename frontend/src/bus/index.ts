import mitt, { Emitter } from "mitt";

type MainBusEvents = {
  before_search: void;
};

type BusType = Emitter<MainBusEvents>;

export const mainBus: BusType = mitt<MainBusEvents>();
