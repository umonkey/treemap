// Global imports.
import mitt, { Emitter } from "mitt";

// Project imports.
import {
  IFileReadyEvent,
  IFileUploadRequest,
  ILatLng,
  IMarkerClickEvent,
} from "@/types";

type MainBusEvents = {
  before_search: void;
  initialize: void;
  pan_to: ILatLng;
  reload_map: void;
  search: string;
  tree_clicked: IMarkerClickEvent;
  upload_finished: void;
  upload_image: IFileUploadRequest;
  upload_progress: number;
  upload_ready: IFileReadyEvent;
};

type BusType = Emitter<MainBusEvents>;

export const mainBus: BusType = mitt<MainBusEvents>();
