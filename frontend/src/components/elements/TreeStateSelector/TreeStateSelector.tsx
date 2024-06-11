/**
 * Tree state selector.
 * https://mui.com/material-ui/react-select/
 */

// Global imports.
import { FormControl, InputLabel, Select, MenuItem, SelectChangeEvent } from "@mui/material";

// Project imports.
import { locale } from "@/locale";

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
      <InputLabel id="tree-state-label">{locale.state()}</InputLabel>
      <Select
        data-testid="tree-state-select"
        labelId="tree-state-label"
        id="tree-state"
        value={props.state}
        onChange={handleChange}
        label="State"
      >
        <MenuItem data-testid="healthy-state" value="healthy">{locale.stateHealthy()}</MenuItem>
        <MenuItem value="deformed">{locale.stateDeformed()}</MenuItem>
        <MenuItem value="sick">{locale.stateSick()}</MenuItem>
        <MenuItem value="dead">{locale.stateDead()}</MenuItem>
        <MenuItem value="stomp">{locale.stateStomp()}</MenuItem>
        <MenuItem value="gone">{locale.stateGone()}</MenuItem>
      </Select>
    </FormControl>
  );
};
