// Global imports.
import { useEffect, useState, useRef } from "react";

// Project imports.
import { mainBus } from "@/bus";
import { ILatLng, IMapView } from "@/types";
import { useMapState } from "@/hooks";

const changed = (a: ILatLng, b: ILatLng) => {
  return a.lat !== b.lat || a.lon !== b.lon;
}

interface IProps {
  center: ILatLng;
  zoom: number;
}

export const useMapBase = (props: IProps) => {
  const [center, setCenter] = useState<ILatLng>(props.center);
  const [zoom] = useState<number>(props.zoom);
  const { setMapState } = useMapState();

  const ref = useRef(null);
  const mapRef = useRef<L.Map>(null);

  // Local center changed, move the map.
  useEffect(() => {
    mapRef.current?.closePopup();
    mapRef.current?.panTo([center.lat, center.lon]);
  }, [center]);

  // Externally supplied center changed, update the local value
  // to trigger map panning.
  useEffect(() => {
    if (changed(props.center, center)) {
      setCenter(props.center);
    }
  }, [center, props.center]);

  useEffect(() => {
    const handler = (center: ILatLng) => {
      mapRef.current?.panTo([center.lat, center.lon]);
    };

    mainBus.on("pan_to", handler);
    return () => mainBus.off("pan_to", handler);
  }, []);

  useEffect(() => {
    if (!ref.current) {
      console.debug("Ref empty, not installing resize observer.");
      return;
    }

    console.debug("Installing resize observer.");

    const resizeObserver = new ResizeObserver(() => {
      console.debug("Map resized, invalidating size.");
      mapRef.current?.invalidateSize();
    });

    resizeObserver.observe(ref.current);

    return () => resizeObserver.disconnect();
  }, [ref]);

  const handleViewChange = (view: IMapView) => {
    setMapState(view);
  };

  return {
    ref,
    mapRef,
    zoom,
    handleViewChange,
  };
};
