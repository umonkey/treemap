// Global imports.
import { useState, useEffect, useMemo, useRef } from "react";

// Local imports.
import { IProps } from "./types";

export const useDraggableMarker = (props: IProps) => {
  const [center, setCenter] = useState(props.center);
  const ref = useRef<L.Marker>(null);

  useEffect(() => {
    setCenter(props.center);
  }, [props.center]);

  const eventHandlers = useMemo(() => ({
    dragend: () => {
      ref.current && props.onChange({
        lat: ref.current.getLatLng().lat,
        lon: ref.current.getLatLng().lng,
      });
    },
  }), [props]);

  return {
    center,
    eventHandlers,
    ref,
  };
};
