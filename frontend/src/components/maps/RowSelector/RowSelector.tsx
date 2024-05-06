// Project imports.
import { DraggableMarker } from "@/components";
import { ILatLng } from "@/types";

// Local imports.
import { useRowSelector } from "./hooks";

interface IProps {
  center: ILatLng;
  onChange: (a: ILatLng, b: ILatLng) => void;
}

export const RowSelector = (props: IProps) => {
  const { center1, center2, handleChange1, handleChange2 } = useRowSelector(props);

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
    </>
  );
};
