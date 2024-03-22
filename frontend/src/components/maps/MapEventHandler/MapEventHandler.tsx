/**
 * A simple wrapper class that makes Leaflet events more usable.
 */

import { useState } from "react";
import { useMapEvents } from "react-leaflet";
import { LatLngBounds } from "leaflet";

import { IBounds, ILatLng } from "@/types";

interface IProps {
  onClick?: (position: ILatLng) => void;
  onBoundsChange?: (bounds: IBounds) => void;
}

export const MapEventHandler = (props: IProps) => {
  const [initialized, setInitialized] = useState(false);

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
    if (!initialized) {
      setInitialized(true);

      console.debug("The map is ready.");

      setTimeout(() => {
        handleBoundsChange(map.getBounds());
      }, 100);
    }
  });

  return null;
};
