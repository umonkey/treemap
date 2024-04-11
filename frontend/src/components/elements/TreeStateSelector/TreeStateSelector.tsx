/**
 * Tree state selector.
 * https://mui.com/material-ui/react-select/
 */

import { FormControl, InputLabel, Select, MenuItem, SelectChangeEvent } from "@mui/material";

interface IProps {
  state: string;
  onChange: (value: string) => void;
}

export const TreeStateSelector = (props: IProps) => {
  const handleChange = (event: SelectChangeEvent<string>) => {
    props.onChange(event.target.value);
  };

  return (
    <FormControl variant="standard">
      <InputLabel id="tree-state-label">State</InputLabel>
      <Select
        data-testid="tree-state-select"
        labelId="tree-state-label"
        id="tree-state"
        value={props.state}
        onChange={handleChange}
        label="State"
      >
        <MenuItem data-testid="healthy-state" value="healthy">Healthy</MenuItem>
        <MenuItem value="deformed">Deformed</MenuItem>
        <MenuItem value="sick">Sick</MenuItem>
        <MenuItem value="dead">Dead</MenuItem>
        <MenuItem value="gone">Gone</MenuItem>
      </Select>
    </FormControl>
  );
};
