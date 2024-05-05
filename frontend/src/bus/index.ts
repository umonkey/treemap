// Global imports.
import mitt, { Emitter } from "mitt";

// Project imports.
import { ILatLng, IFileUploadRequest, IMarkerClickEvent } from "@/types";

type MainBusEvents = {
  before_search: void;
  tree_clicked: IMarkerClickEvent;
  pan_to: ILatLng;
  upload_image: IFileUploadRequest;
  upload_progress: number;
  upload_finished: void;
};

type BusType = Emitter<MainBusEvents>;

export const mainBus: BusType = mitt<MainBusEvents>();
