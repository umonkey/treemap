/**
 * A marker with a crosshairs icon that's always in the center of the map.
 */

import { useMapEvents } from "react-leaflet";
import { ILatLng } from "@/types";
import CenterIcon from "./icons/marker-icon.svg";
import "./style.css";

interface IProps {
  onChange: (position: ILatLng) => void;
}

export const LocationPicker = (props: IProps) => {
  const map = useMapEvents({
    move: () => {
      const ll = {
        lat: map.getCenter().lat,
        lon: map.getCenter().lng,
      } as ILatLng;

      props.onChange(ll);
    },
  });

  return (
    <img src={CenterIcon} alt="Crosshairs" className="crosshairs" />
  );
};
