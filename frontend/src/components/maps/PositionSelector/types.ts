import { ILatLng } from "@/types";

export interface IProps {
  onChange: (points: ILatLng[]) => void;
}
