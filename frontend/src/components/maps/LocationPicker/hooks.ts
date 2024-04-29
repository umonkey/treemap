import { useRef } from "react";
import { useMapEvents } from "react-leaflet";

import { ILatLng } from "@/types";

const DELAY = 50;

interface IProps {
  onChange: (position: ILatLng) => void;
}

export const useLocationPicker = (props: IProps) => {
  const timeoutId = useRef<ReturnType<typeof setTimeout>|null>(null);

  const updateLocation = (position: ILatLng) => {
    if (timeoutId.current) {
      clearTimeout(timeoutId.current);
    }

    timeoutId.current = setTimeout(() => {
      props.onChange(position);
    }, DELAY);
  };

  const map = useMapEvents({
    move: () => {
      const ll = {
        lat: map.getCenter().lat,
        lon: map.getCenter().lng,
      } as ILatLng;

      updateLocation(ll);
    },
  });
};
