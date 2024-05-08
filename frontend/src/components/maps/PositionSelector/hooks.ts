// Global imports.
import { useEffect, useState } from "react";

// Project imports.
import { useMapState } from "@/hooks";
import { ILatLng } from "@/types";

// Local imports.
import { IProps } from "./types";

export const rowBuilder = (a: ILatLng, b: ILatLng, n: number): ILatLng[] => {
  if (n < 3) {
    return [a, b];
  }

  const latDiff = (b.lat - a.lat) / (n - 1);
  const lngDiff = (b.lon - a.lon) / (n - 1);

  const positions = [a];

  for (let i = 1; i < n; i++) {
    positions.push({
      lat: a.lat + latDiff * i,
      lon: a.lon + lngDiff * i,
    });
  }

  return positions;
};

export const usePositionSelector = (props: IProps) => {
  const { mapState } = useMapState();
  const [number, setNumber] = useState(0);
  const [singlePosition, setSinglePosition] = useState<ILatLng>(mapState.center);
  const [rowStart, setRowStart] = useState<ILatLng>(mapState.center);
  const [rowEnd, setRowEnd] = useState<ILatLng>(mapState.center);
  const [dots, setDots] = useState<ILatLng[]>([]);

  useEffect(() => {
    if (number === 1) {
      setDots([singlePosition]);
    } else if (number > 1) {
      setDots(rowBuilder(rowStart, rowEnd, number));
    }
  }, [number, singlePosition, rowStart, rowEnd]);

  useEffect(() => {
    props.onChange(dots);
  }, [dots, props]);

  const handleNumberChange = (value: number) => {
    setNumber(value);
  };

  const handleSinglePositionChange = (position: ILatLng) => {
    setSinglePosition(position);
  };

  const handleRowChange = (a: ILatLng, b: ILatLng) => {
    setRowStart(a);
    setRowEnd(b);
  };

  return {
    dots,
    handleNumberChange,
    handleRowChange,
    handleSinglePositionChange,
    mapState,
    number,
    singlePosition,
  };
};
