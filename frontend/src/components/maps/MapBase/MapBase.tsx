/**
 * This is the very basic Leaflet map wrapper.
 *
 * Loads the styles, initializes layers, that's it.
 */

import { useEffect, useRef } from "react";
import { MapContainer } from "react-leaflet";
import { LayerSelector } from "@/components";
import { ILatLng } from "@/types";

import "leaflet/dist/leaflet.css";

interface IProps {
  center: ILatLng;
  children?: React.ReactNode | React.ReactNode[];
}

export const MapBase = (props: IProps) => {
  const ref = useRef(null);

  useEffect(() => {
    if (!ref.current) {
      console.debug("Ref empty, not installing resize observer.");
      return;
    }

    console.debug("Installing resize observer.", ref.current);

    const resizeObserver = new ResizeObserver(() => {
      console.debug("Map resized!");
    });

    resizeObserver.observe(ref.current);

    return () => resizeObserver.disconnect();
  }, [ref]);

  return (
    <div ref={ref} style={{
      height: "100%",
      width: "100%",
    }}>
      <MapContainer center={[props.center.lat, props.center.lon]} zoom={13} maxZoom={25} scrollWheelZoom={true} className="map" zoomControl={false}>
        <LayerSelector />

        {props.children}
      </MapContainer>
    </div>
  );
};
