// Global imports.
import { useEffect, useState } from "react";

interface IProps {
  onChange: (value: number) => void;
}

export const useTreeCountSelector = (props: IProps) => {
  const [mode, setMode] = useState<string>("single");
  const [number, setNumber] = useState<number | null>(5);

  useEffect(() => {
    if (mode === "single") {
      props.onChange(1);
    } else if (number && number >= 2) {
      props.onChange(number);
    }
  }, [mode, number, props]);

  const handleModeChange = (
    // @ts-expect-error TS6133
    event: React.MouseEvent<HTMLElement>,
    value: string | null,
  ) => {
    value && setMode(value);
  };

  const handleNumberChange = (
    event: React.ChangeEvent<HTMLInputElement>,
  ) => {
    if (event.target.value) {
      setNumber(parseInt(event.target.value));
    } else {
      setNumber(null);
    }
  };

  return {
    mode,
    number,
    handleModeChange,
    handleNumberChange,
    singleLabel: mode === "single" ? "Single tree" : "Single",
  };
};
