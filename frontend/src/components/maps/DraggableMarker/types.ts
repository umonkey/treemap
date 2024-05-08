import { ILatLng } from "@/types";

export interface IProps {
  center: ILatLng;
  onChange: (center: ILatLng) => void;
}
