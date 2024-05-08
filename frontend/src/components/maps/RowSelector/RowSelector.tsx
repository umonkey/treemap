// Global imports.
import { Polyline } from "react-leaflet";

// Project imports.
import { DraggableMarker } from "@/components";

// Local imports.
import { useRowSelector } from "./hooks";
import { IProps } from "./types";

export const RowSelector = (props: IProps) => {
  const { center1, center2, handleChange1, handleChange2, path } = useRowSelector(props);

  return (
    <>
      <DraggableMarker
        center={center1}
        onChange={handleChange1}
      />

      <DraggableMarker
        center={center2}
        onChange={handleChange2}
      />

      <Polyline
        pathOptions={{ color: "green" }}
        positions={path}
      />
    </>
  );
};
