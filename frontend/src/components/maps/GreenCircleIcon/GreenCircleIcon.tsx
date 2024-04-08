/**
 * Custom marker icon for the map.
 *
 * @docs https://leafletjs.com/examples/custom-icons/
 */

import marker from "./icons/marker-icon.svg";

import { Icon } from "leaflet";

export const GreenCircleIcon = new Icon({
  iconUrl: marker,
  iconSize: [20, 20],
  iconAnchor: [10, 10],
});
