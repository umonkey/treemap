/**
 * This is the very basic Leaflet map wrapper.
 *
 * Loads the styles, initializes base layers, that's it.
 * Everything else is up to the children.
 */

// Global imports.
import { MapContainer } from "react-leaflet";
import "leaflet/dist/leaflet.css";

// Project imports.
import { MapEventHandler, LayerSelector } from "@/components";
import { ILatLng } from "@/types";

// Local imports.
import { useMapBase } from "./hooks";
import "./styles.scss";

interface IProps {
  center: ILatLng;
  zoom: number;
  children?: React.ReactNode | React.ReactNode[];
}

export const MapBase = (props: IProps) => {
  const {
    handleViewChange,
    handleZoomChange,
    mapRef,
    maxZoom,
    ref,
    zoom,
  } = useMapBase({
    center: props.center,
    zoom: props.zoom,
  });

  return (
    <div ref={ref} style={{
      height: "100%",
      width: "100%",
    }}>
      <MapContainer ref={mapRef} center={[props.center.lat, props.center.lon]} zoom={zoom} maxZoom={maxZoom} scrollWheelZoom={true} className="map" zoomControl={false}>
        <MapEventHandler onViewChange={handleViewChange} />
        <LayerSelector onZoomChange={handleZoomChange} />

        {props.children}
      </MapContainer>
    </div>
  );
};
