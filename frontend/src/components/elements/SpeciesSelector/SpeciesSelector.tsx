// Global imports.
import { Autocomplete, TextField } from '@mui/material';

// Project imports.
import { locale } from "@/locale";

// Local imports.
import { useSpeciesSelector } from "./hooks";
import "./styles.scss";

interface IProps {
  value?: string;
  onChange: (value: string) => void;
  required?: boolean;
}

export const SpeciesSelector = (props: IProps) => {
  const required = props.required || true;

  const { currentValue, setCurrentValue, options, handleChange, handleInputChange, renderOption, recent } = useSpeciesSelector({
    onChange: props.onChange,
    default: props.value || "",
  });

  const handleHintClick = (hint: string) => {
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
        renderInput={(params) => <TextField {...params} label={locale.species()} required={!!required} />}
        renderOption={renderOption}
        onChange={handleChange}
        onInputChange={handleInputChange}
        value={currentValue}
      />

      {recent.length > 0 && (
        <div className="recent">{locale.speciesRecent()}: {recent.slice(0, 10).map((n, idx) => {
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
