/**
 * Custom marker icon for the map.
 *
 * @docs https://leafletjs.com/examples/custom-icons/
 */

import icon from "@/icons/dot-yellow.svg";

import { Icon } from "leaflet";

export const YellowCircleIcon = new Icon({
  iconUrl: icon,
  iconSize: [20, 20],
  iconAnchor: [10, 10],
});
