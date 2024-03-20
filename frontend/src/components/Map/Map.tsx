import { MapContainer, TileLayer, Marker, Popup, ZoomControl, useMapEvents } from "react-leaflet";

import { useMarkers } from "./hooks";
import { ITreeInfo } from "../../services/api/types";

import "./styles.css";

export const Map = () => {
  const { center, markers, reload } = useMarkers();

  const MapEventHandler = () => {
    const map = useMapEvents({
      click: () => {
        console.debug("MAP CLICK");
      },

      load: () => {
        reload(map.getBounds());
      },

      zoomend: () => {
        reload(map.getBounds());
      },

      moveend: () => {
        reload(map.getBounds());
      },
    });

    /**
     * This causes recursion, use a timer for now.
     */
    map.whenReady(() => {
      setTimeout(() => {
        reload(map.getBounds());
      }, 100);
    });

    return null;
  };

  const Markers = () => {
    const items = markers.map((marker: ITreeInfo, index: number) => (
      <Marker key={index} position={[marker.lat, marker.lon]}>
        <Popup>
          A pretty CSS3 popup. <br /> Easily customizable.
        </Popup>
      </Marker>
    ));

    return items;
  };

  return (
    <MapContainer center={center} zoom={13} maxZoom={18} scrollWheelZoom={true} className="map">
      <MapEventHandler />

      <TileLayer
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
      />

      <Markers />

      <ZoomControl position="bottomright" />
    </MapContainer>
  );
};
