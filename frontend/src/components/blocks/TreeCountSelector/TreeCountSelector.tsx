/**
 * A complex UI element to select one or multiple trees to add.
 */

// Global imports.
import { ToggleButton, ToggleButtonGroup, TextField } from "@mui/material";

// Local imports.
import { useTreeCountSelector } from "./hooks";
import "./styles.scss";

interface IProps {
  onChange: (value: number) => void;
}

export const TreeCountSelector = (props: IProps) => {
  const {
    handleModeChange,
    handleNumberChange,
    mode,
    number,
    singleLabel,
  } = useTreeCountSelector(props);

  return (
    <div className="TreeCountSelector">
      <ToggleButtonGroup exclusive value={mode} onChange={handleModeChange} aria-label="number of trees">
        <ToggleButton color="primary" value="single" aria-label="single">{singleLabel}</ToggleButton>
        <ToggleButton value="row" aria-label="row">Row</ToggleButton>
      </ToggleButtonGroup>

      {mode === "row" && (
        <TextField
          id="number"
          label="Number of trees"
          variant="standard"
          type="number"
          value={number}
          onChange={handleNumberChange}
          inputProps={{ min: 2, max: 100 }}
        />
      )}
    </div>
  );
};
