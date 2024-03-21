import { LayersControl, MapContainer, TileLayer, Marker, Popup, ZoomControl, useMapEvents } from "react-leaflet";
import { LatLngBounds } from "leaflet";

import { IBounds, ILatLng, ITreeInfo } from "../../../types";
import { LocateControl } from "../LocateControl";
import { AddTreeControl } from "../AddTreeControl";

import "leaflet/dist/leaflet.css";

interface IProps {
  center: ILatLng;
  onAddTree: () => void;
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
        <Popup>{marker.name}</Popup>
      </Marker>
    ));

    return items;
  };

  return (
    <MapContainer center={[props.center.lat, props.center.lon]} zoom={13} maxZoom={18} scrollWheelZoom={true} className="map" zoomControl={false}>
      <MapEventHandler />

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

      <Markers />

      <ZoomControl position="bottomright" />
      <LocateControl position="bottomright" />
      <AddTreeControl position="bottomright" onClick={props.onAddTree} />
    </MapContainer>
  );
};