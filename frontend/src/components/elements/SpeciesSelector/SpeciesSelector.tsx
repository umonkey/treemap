// Global imports.
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
  const { value } = props;
  const required = props.required || true;

  const { options, handleChange, handleInputChange, renderOption } = useSpeciesSelector({
    onChange: props.onChange,
  });

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
        value={value}
      />
    </div>
  );
};
