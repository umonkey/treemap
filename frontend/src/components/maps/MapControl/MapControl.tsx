import { MapContainer, TileLayer, Marker, Popup, ZoomControl, useMapEvents } from "react-leaflet";
import { LatLngBounds } from "leaflet";

import { IBounds, ILatLng, ITreeInfo } from "../../../types";
import { LocateControl } from "../LocateControl";

import "leaflet/dist/leaflet.css";

interface IProps {
  center: ILatLng;
  onBoundsChange?: (bounds: IBounds) => void;
  markers?: ITreeInfo[];
}

export const MapControl = (props: IProps) => {
  const handleBoundsChange = (bounds: LatLngBounds) => {
    props.onBoundsChange && props.onBoundsChange({
      north: bounds.getNorth(),
      east: bounds.getEast(),
      south: bounds.getSouth(),
      west: bounds.getWest(),
    });
  };

  const MapEventHandler = () => {
    const map = useMapEvents({
      click: () => {
        handleBoundsChange(map.getBounds());
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

    /**
     * This causes recursion, use a timer for now.
     */
    map.whenReady(() => {
      setTimeout(() => {
        handleBoundsChange(map.getBounds());
      }, 100);
    });

    return null;
  };

  const Markers = () => {
    if (!props.markers) {
      return null;
    }

    const items = props.markers?.map((marker: ITreeInfo, index: number) => (
      <Marker key={index} position={[marker.lat, marker.lon]}>
        <Popup>
          A pretty CSS3 popup. <br /> Easily customizable.
        </Popup>
      </Marker>
    ));

    return items;
  };

  return (
    <MapContainer center={[props.center.lat, props.center.lon]} zoom={13} maxZoom={18} scrollWheelZoom={true} className="map" zoomControl={false}>
      <MapEventHandler />

      <TileLayer
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
      />

      <Markers />

      <ZoomControl position="bottomright" />
      <LocateControl position="bottomright" />
    </MapContainer>
  );
};
