import { LayersControl, TileLayer } from "react-leaflet";

export const LayerSelector = () => {
  return (
    <LayersControl position="topright">
      <LayersControl.BaseLayer checked name="OpenStreetMap">
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        />
      </LayersControl.BaseLayer>
      <LayersControl.BaseLayer name="Google Maps">
        <TileLayer
          attribution='&copy; Google Maps'
          url="http://{s}.google.com/vt/lyrs=s,h&x={x}&y={y}&z={z}"
          subdomains={["mt0", "mt1", "mt2", "mt3"]}
        />
      </LayersControl.BaseLayer>
    </LayersControl>
  );
};
