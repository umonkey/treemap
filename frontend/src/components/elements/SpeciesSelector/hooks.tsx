import { useEffect, useState } from "react";

import { treeMapService } from "@/services/api";

interface IOption {
  label: string;
  hint: string;
}

interface IProps {
  default: string;
  onChange: (value: string) => void;
}

export const useSpeciesSelector = (props: IProps) => {
  const [currentValue, setCurrentValue] = useState<string>(props.default);

  const [options, setOptions] = useState<IOption[]>([]);
  const [recent, setRecent] = useState<string[]>([]);

  useEffect(() => {
    setCurrentValue(props.default);
  }, [props.default]);

  useEffect(() => {
    (async () => {
      try {
        const res = await treeMapService.suggestSpecies();
        setRecent([...res.slice(0, 9), "Unknown"]);
      } catch (e) {
        console.error("Error reading recent species.", e);
        setRecent([]);
      }
    })();
  }, []);

  // @ts-expect-error TS6133
  const handleChange = (event: React.SyntheticEvent<Element, Event>, value: string | IOption) => {
    if (typeof value === "string") {
      props.onChange(value);
    } else {
      props.onChange(value.label);
    }
  };

  // @ts-expect-error TS6133
  const handleInputChange = async (event: React.SyntheticEvent, value: string) => {
    props.onChange(value);

    const res = await treeMapService.searchSpecies(value);

    const newOptions = res.map((species) => ({
      label: species.name,
      hint: species.local,
    }));

    console.debug(`Received ${newOptions.length} options for ${value}`);

    setOptions(newOptions);
  }

  const renderOption = (props: React.HTMLAttributes<HTMLLIElement>, option: IOption) => {
    return (
      <li {...props}>
        <div className="singleOption">
          <div className="label">{option.label}</div>
          <div className="hint">{option.hint}</div>
        </div>
      </li>
    );
  };

  return {
    currentValue,
    options,
    handleChange,
    handleInputChange,
    renderOption,
    recent,
    setCurrentValue,
  };
};
