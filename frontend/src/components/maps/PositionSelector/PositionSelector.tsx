/**
 * A complex UI element to select one or multiple positions.
 *
 * Shows a map, lets the user select one or multiple points,
 * reports them via the callback.  More UI features can be added later.
 */

// Project imports.
import { Marker } from "react-leaflet";
import { GreenCircleIcon, LocationPicker, MapBase, RowSelector, TreeCountSelector } from "@/components";

// Local imports.
import { usePositionSelector } from "./hooks";
import { IProps } from "./types";
import "./styles.scss";

export const PositionSelector = (props: IProps) => {
  const {
    center,
    dots,
    handleNumberChange,
    handleRowChange,
    handleSinglePositionChange,
    mapState,
    number,
  } = usePositionSelector(props);

  return (
    <div className="PositionSelector">
      <TreeCountSelector
        onChange={handleNumberChange}
      />

      <MapBase center={mapState.center} zoom={mapState.zoom}>
        {number === 1 && (
          <LocationPicker
            onChange={handleSinglePositionChange}
          />
        )}

        {number > 1 && (
          <RowSelector
            center={center}
            onChange={handleRowChange}
          />
        )}

        {dots.map((dot, index) => (
          <Marker
            key={index}
            position={[dot.lat, dot.lon]}
            icon={GreenCircleIcon}
          />
        ))}
      </MapBase>

    </div>
  );
};
