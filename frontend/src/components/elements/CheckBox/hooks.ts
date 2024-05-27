// Global imports.
import { useState, useEffect } from "react";

// Local imports.
import { IProps } from "./types";

export const useCheckBox = (props: IProps) => {
  const [value, setValue] = useState<boolean>(props.value);

  useEffect(() => {
    setValue(props.value);
  }, [props.value]);

  useEffect(() => {
    props.onChange(value);
  }, [props, value]);

  const handleClick = () => {
    setValue(!value);
  };

  return {
    label: props.label,
    tick: value ? "✓" : "",
    handleClick,
  };
}
