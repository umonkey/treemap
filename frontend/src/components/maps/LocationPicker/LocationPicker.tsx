/**
 * A marker with a crosshairs icon that's always in the center of the map.
 */

import { useState, useMemo } from "react";
import { Marker, useMapEvents } from "react-leaflet";
import { Icon } from "leaflet";
import { ILatLng } from "@/types";

import marker from "./icons/marker-icon.svg";

interface IProps {
  onChange: (position: ILatLng) => void;
}

export const LocationPicker = (props: IProps) => {
  const [center, setCenter] = useState<ILatLng | null>(null);

  const map = useMapEvents({
    move: () => {
      const ll = {
        lat: map.getCenter().lat,
        lon: map.getCenter().lng,
      } as ILatLng;

      setCenter(ll);
      props.onChange(ll);
    },
  });

  const icon = useMemo(() => {
    return new Icon({
      iconUrl: marker,
      iconSize: [30, 30],
      iconAnchor: [15, 15],
    });
  }, []);

  if (!center) {
    return null;
  }

  return (
    <Marker position={{
      lat: center.lat,
      lng: center.lon,
    }} icon={icon}>
    </Marker>
  );
};
