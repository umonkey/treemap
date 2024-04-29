/**
 * A marker with a crosshairs icon that's always in the center of the map.
 */

import { ILatLng } from "@/types";

// Local imports.
import CenterIcon from "./icons/marker-icon.svg";
import "./style.css";
import { useLocationPicker } from "./hooks";

interface IProps {
  onChange: (position: ILatLng) => void;
}

export const LocationPicker = (props: IProps) => {
  useLocationPicker(props);

  return (
    <img src={CenterIcon} alt="Crosshairs" className="crosshairs" />
  );
};
