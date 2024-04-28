// Global imports.
import { useState } from "react";
import { Autocomplete, TextField } from '@mui/material';

// Local imports.
import { useSpeciesSelector } from "./hooks";
import "./styles.scss";

interface IProps {
  value?: string;
  onChange: (value: string) => void;
  required?: boolean;
}

export const SpeciesSelector = (props: IProps) => {
  const [currentValue, setCurrentValue] = useState<string>(props.value || "");
  const required = props.required || true;

  const { options, handleChange, handleInputChange, renderOption, recent } = useSpeciesSelector({
    onChange: props.onChange,
  });

  const handleHintClick = (hint: string) => {
    console.debug(`Hint clicked: ${hint}`);
    setCurrentValue(hint);
  };

  return (
    <div className="SpeciesSelector">
      <Autocomplete
        disableClearable
        disablePortal
        freeSolo
        filterOptions={(x) => x}
        id="species-selector"
        options={options}
        renderInput={(params) => <TextField {...params} label="Species" required={!!required} />}
        renderOption={renderOption}
        onChange={handleChange}
        onInputChange={handleInputChange}
        value={currentValue}
      />

      {recent.length >= 3 && (
        <div className="recent">Recent species: {recent.slice(0, 3).map((n, idx) => {
          const items = [];

          if (idx > 0) {
            items.push(", ");
          }

          items.push(<span onClick={() => {
            handleHintClick(n);
          }} key={idx}><u>{n}</u></span>);

          return items;
        })}.</div>
      )}
    </div>
  );
};
