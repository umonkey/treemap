/**
 * A complex UI element to select one or multiple trees to add.
 */

// Global imports.
import { ToggleButton, ToggleButtonGroup, TextField } from "@mui/material";

// Project imports.
import { locale } from "@/locale";

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
        <ToggleButton value="row" aria-label="row">{locale.treeRowLabel()}</ToggleButton>
      </ToggleButtonGroup>

      {mode === "row" && (
        <TextField
          id="number"
          label={locale.numberOfTrees()}
          variant="standard"
          type="number"
          value={number}
          onChange={handleNumberChange}
          inputProps={{ min: 0, max: 100 }}
        />
      )}
    </div>
  );
};
