// Global imports.
import { useState, useEffect, useMemo, useRef } from "react";

// Project imports.
import { ILatLng } from "@/types";

interface IProps {
  center: ILatLng;
  onChange: (center: ILatLng) => void;
}

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
