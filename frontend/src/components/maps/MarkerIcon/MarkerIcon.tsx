/**
 * Custom marker icon for the map.
 *
 * @docs https://leafletjs.com/examples/custom-icons/
 */

import marker from "./icons/marker-icon.svg";
import shadow from "./icons/marker-shadow.svg";

import { Icon } from "leaflet";

export const MarkerIcon = new Icon({
  iconUrl: marker,
  iconSize: [25, 41],
  iconAnchor: [12, 41],

  shadowUrl: shadow,
  shadowSize: [41, 41],
  shadowAnchor: [12, 41],
});
