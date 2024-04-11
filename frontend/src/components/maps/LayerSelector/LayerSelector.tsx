import { LayersControl, TileLayer } from "react-leaflet";
import VectorTileLayer from "react-leaflet-vector-tile-layer";
import { getMapTilerKey } from "@/utils/env";

interface IProps {
  onZoomChange: (zoom: number) => void;
}

export const LayerSelector = (props: IProps) => {
  const mapTilerKey = getMapTilerKey();

  const handleMaxZoomChange = (zoom: number) => {
    props.onZoomChange(zoom);
  };

  return (
    <LayersControl position="topright">
      <LayersControl.BaseLayer checked name="MapTiler (vector)">
        <VectorTileLayer
          attribution='&copy; <a href="https://github.com/umonkey/treemap">Tree Map</a> contributors'
          styleUrl={`https://api.maptiler.com/maps/streets-v2/style.json?key=${mapTilerKey}`}
          accessToken={mapTilerKey}
          eventHandlers={{
            add: () => handleMaxZoomChange(25),
          }}
        />
      </LayersControl.BaseLayer>

      <LayersControl.BaseLayer name="OpenStreetMap">
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
          eventHandlers={{
            add: () => handleMaxZoomChange(18),
          }}
        />
      </LayersControl.BaseLayer>

      <LayersControl.BaseLayer name="Google Maps">
        <TileLayer
          attribution='&copy; Google Maps'
          url="http://{s}.google.com/vt/lyrs=s,h&x={x}&y={y}&z={z}"
          subdomains={["mt0", "mt1", "mt2", "mt3"]}
          eventHandlers={{
            add: () => handleMaxZoomChange(18),
          }}
        />
      </LayersControl.BaseLayer>
    </LayersControl>
  );
};
