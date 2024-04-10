/**
 * This is the very basic Leaflet map wrapper.
 *
 * Loads the styles, initializes layers, that's it.
 */

import { useEffect, useState, useRef } from "react";
import { MapContainer } from "react-leaflet";
import { LayerSelector } from "@/components";
import { ILatLng } from "@/types";

import "leaflet/dist/leaflet.css";

interface IProps {
  center: ILatLng;
  children?: React.ReactNode | React.ReactNode[];
}

export const MapBase = (props: IProps) => {
  const [maxZoom, setMaxZoom] = useState<number>(18);

  const ref = useRef(null);
  const mapRef = useRef(null);

  // react-leaflet does not update properties dynamically, so we need to do it manually.
  useEffect(() => {
    if (mapRef.current) {
      // @ts-expect-error TS2339
      mapRef.current.setMaxZoom(maxZoom);
    }
  }, [maxZoom]);

  useEffect(() => {
    if (!ref.current) {
      console.debug("Ref empty, not installing resize observer.");
      return;
    }

    console.debug("Installing resize observer.");

    const resizeObserver = new ResizeObserver(() => {
      console.debug("Map resized, invalidating size.");

      // @ts-expect-error TS18047
      mapRef.current.invalidateSize();
    });

    resizeObserver.observe(ref.current);

    return () => resizeObserver.disconnect();
  }, [ref]);

  const handleZoomChange = (zoom: number) => {
    console.debug(`Max zoom changed to ${zoom}.`);
    setMaxZoom(zoom);
  };

  return (
    <div ref={ref} style={{
      height: "100%",
      width: "100%",
    }}>
      <MapContainer ref={mapRef} center={[props.center.lat, props.center.lon]} zoom={13} maxZoom={maxZoom} scrollWheelZoom={true} className="map" zoomControl={false}>
        <LayerSelector onZoomChange={handleZoomChange} />

        {props.children}
      </MapContainer>
    </div>
  );
};
