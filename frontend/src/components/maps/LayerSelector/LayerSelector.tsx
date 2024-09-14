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
    <LayersControl position="topright" autoZIndex={false}>
      <LayersControl.BaseLayer checked={mapLayer === "MapTiler (vector)"} name="MapTiler (vector)">
        <VectorTileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright" target="_blank">OSM</a> &amp; <a href="https://github.com/umonkey/treemap/wiki/Data-contribution" target="_blank">Tree Map</a> contributors'
          styleUrl={`https://api.maptiler.com/maps/streets-v2/style.json?key=${mapTilerKey}`}
          accessToken={mapTilerKey}
          zIndex={5}
        />
      </LayersControl.BaseLayer>

      <LayersControl.BaseLayer checked={mapLayer === "OpenStreetMap"} name="OpenStreetMap">
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright" target="_blank">OSM</a> &amp; <a href="https://github.com/umonkey/treemap/wiki/Data-contribution" target="_blank">Tree Map</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
          maxZoom={25}
          maxNativeZoom={19}
          zIndex={5}
        />
      </LayersControl.BaseLayer>

      <LayersControl.BaseLayer checked={mapLayer === "Google Maps"} name="Google Maps">
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright" target="_blank">OSM</a> &amp; Google &amp; <a href="https://github.com/umonkey/treemap/wiki/Data-contribution" target="_blank">Tree Map</a> contributors'
          url="http://{s}.google.com/vt/lyrs=s&x={x}&y={y}&z={z}"
          subdomains={["mt0", "mt1", "mt2", "mt3"]}
          maxZoom={25}
          maxNativeZoom={22}
          zIndex={5}
        />
      </LayersControl.BaseLayer>

      <LayersControl.BaseLayer checked={mapLayer === "MapTiler (raster)"} name="MapTiler (raster)">
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright" target="_blank">OSM</a> &amp; MapTiler &amp; <a href="https://github.com/umonkey/treemap/wiki/Data-contribution" target="_blank">Tree Map</a> contributors'
          url={`https://api.maptiler.com/maps/basic-v2/{z}/{x}/{y}.png?key=${mapTilerKey}`}
          maxZoom={25}
          maxNativeZoom={19}
          tileSize={512}
          zoomOffset={-1}
          crossOrigin={true}
          zIndex={5}
        />
      </LayersControl.BaseLayer>

      <LayersControl.Overlay checked={true} name="Drone (50%)">
        <TileLayer
          attribution=""
          tms={true}
          url={`https://treemap-tiles.fra1.digitaloceanspaces.com/{z}/{x}/{y}.png`}
          minZoom={15}
          maxZoom={25}
          maxNativeZoom={21}
          zIndex={10}
          opacity={0.5}
        />
      </LayersControl.Overlay>

      <LayersControl.Overlay name="Drone (90%)">
        <TileLayer
          attribution=""
          tms={true}
          url={`https://treemap-tiles.fra1.digitaloceanspaces.com/{z}/{x}/{y}.png`}
          minZoom={15}
          maxZoom={25}
          maxNativeZoom={21}
          zIndex={12}
          opacity={0.9}
        />
      </LayersControl.Overlay>

    </LayersControl>
  );
};
