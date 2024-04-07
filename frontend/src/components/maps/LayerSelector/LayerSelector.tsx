import { LayersControl, TileLayer } from "react-leaflet";
import VectorTileLayer from "react-leaflet-vector-tile-layer";
import { getMapTilerKey } from "@/utils/env";

export const LayerSelector = () => {
  const mapTilerKey = getMapTilerKey();

  return (
    <LayersControl position="topright">
      <LayersControl.BaseLayer checked name="MapTiler (vector)" maxZoom={25}>
        <VectorTileLayer
          styleUrl={`https://api.maptiler.com/maps/streets-v2/style.json?key=${mapTilerKey}`}
          accessToken={mapTilerKey}
        />
      </LayersControl.BaseLayer>

      <LayersControl.BaseLayer name="OpenStreetMap" maxZoom={18}>
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        />
      </LayersControl.BaseLayer>

      <LayersControl.BaseLayer name="Google Maps" maxNativeZoom={18} maxZoom={18}>
        <TileLayer
          attribution='&copy; Google Maps'
          url="http://{s}.google.com/vt/lyrs=s,h&x={x}&y={y}&z={z}"
          subdomains={["mt0", "mt1", "mt2", "mt3"]}
        />
      </LayersControl.BaseLayer>
    </LayersControl>
  );
};
