// Global imports.
import { LayersControl, TileLayer } from "react-leaflet";
import VectorTileLayer from "react-leaflet-vector-tile-layer";

// Local imports.
import { useLayerSelector } from "./hooks";

export const LayerSelector = () => {
  const {
    mapTilerKey,
    mapLayer,
  } = useLayerSelector();

  return (
    <LayersControl position="topright">
      <LayersControl.BaseLayer checked={mapLayer === "MapTiler (vector)"} name="MapTiler (vector)">
        <VectorTileLayer
          attribution='&copy; <a href="https://github.com/umonkey/treemap/wiki/Data-contribution" target="_blank">Tree Map</a> contributors'
          styleUrl={`https://api.maptiler.com/maps/streets-v2/style.json?key=${mapTilerKey}`}
          accessToken={mapTilerKey}
        />
      </LayersControl.BaseLayer>

      <LayersControl.BaseLayer checked={mapLayer === "OpenStreetMap"} name="OpenStreetMap">
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
          maxZoom={25}
          maxNativeZoom={19}
        />
      </LayersControl.BaseLayer>

      <LayersControl.BaseLayer checked={mapLayer === "Google Maps"} name="Google Maps">
        <TileLayer
          attribution='&copy; Google Maps'
          url="http://{s}.google.com/vt/lyrs=s&x={x}&y={y}&z={z}"
          subdomains={["mt0", "mt1", "mt2", "mt3"]}
          maxZoom={25}
          maxNativeZoom={22}
        />
      </LayersControl.BaseLayer>

      <LayersControl.BaseLayer checked={mapLayer === "MapTiler (raster)"} name="MapTiler (raster)">
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url={`https://api.maptiler.com/maps/basic-v2/{z}/{x}/{y}.png?key=${mapTilerKey}`}
          maxZoom={25}
          maxNativeZoom={19}
          tileSize={512}
          zoomOffset={-1}
          crossOrigin={true}
        />
      </LayersControl.BaseLayer>

    </LayersControl>
  );
};
