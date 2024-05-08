import { ILatLng } from "@/types";

export interface IProps {
  center: ILatLng;
  onChange: (a: ILatLng, b: ILatLng) => void;
}
