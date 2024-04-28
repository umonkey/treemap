/**
 * A simple wrapper class that makes Leaflet events more usable.
 */

import { useState } from "react";
import { useMapEvents } from "react-leaflet";

import { IBounds, ILatLng, IMapView } from "@/types";

interface IProps {
  onClick?: (position: ILatLng) => void;
  onBoundsChange?: (bounds: IBounds) => void;
  onViewChange?: (view: IMapView) => void;
}

export const MapEventHandler = (props: IProps) => {
  const [initialized, setInitialized] = useState(false);

  const reportViewChange = () => {
    const bounds = map.getBounds();
    const center = map.getCenter();

    props.onBoundsChange && props.onBoundsChange({
      north: bounds.getNorth(),
      east: bounds.getEast(),
      south: bounds.getSouth(),
      west: bounds.getWest(),
    });

    const view = {
      center: {
        lat: center.lat,
        lon: center.lng,
      },
      zoom: map.getZoom(),
      bounds: {
        north: bounds.getNorth(),
        east: bounds.getEast(),
        south: bounds.getSouth(),
        west: bounds.getWest(),
      },
    } as IMapView;

    props.onViewChange && props.onViewChange(view);
  };

  const map = useMapEvents({
    click: (e) => {
      props.onClick && props.onClick({
        lat: e.latlng.lat,
        lon: e.latlng.lng,
      });
    },

    load: () => {
      reportViewChange();
    },

    zoomend: () => {
      reportViewChange();
      console.debug(`Map zoom changed to ${map.getZoom()}.`);
    },

    moveend: () => {
      reportViewChange();
    },
  });

  map.whenReady(() => {
    if (!initialized) {
      setInitialized(true);

      console.debug("The map is ready.");

      setTimeout(() => {
        reportViewChange();
      }, 100);
    }
  });

  return null;
};
