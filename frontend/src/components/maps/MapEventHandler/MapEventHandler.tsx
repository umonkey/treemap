/**
 * A simple wrapper class that makes Leaflet events more usable.
 */

import { useMapEvents } from "react-leaflet";
import { LatLngBounds } from "leaflet";

import { IBounds, ILatLng } from "@/types";

interface IProps {
  onClick?: (position: ILatLng) => void;
  onBoundsChange?: (bounds: IBounds) => void;
}

export const MapEventHandler = (props: IProps) => {
  const handleBoundsChange = (bounds: LatLngBounds) => {
    props.onBoundsChange && props.onBoundsChange({
      north: bounds.getNorth(),
      east: bounds.getEast(),
      south: bounds.getSouth(),
      west: bounds.getWest(),
    });
  };

  const map = useMapEvents({
    click: (e) => {
      props.onClick && props.onClick({
        lat: e.latlng.lat,
        lon: e.latlng.lng,
      });
    },

    load: () => {
      handleBoundsChange(map.getBounds());
    },

    zoomend: () => {
      handleBoundsChange(map.getBounds());
    },

    moveend: () => {
      handleBoundsChange(map.getBounds());
    },
  });

  map.whenReady(() => {
    setTimeout(() => {
      handleBoundsChange(map.getBounds());
    }, 100);
  });

  return null;
};
