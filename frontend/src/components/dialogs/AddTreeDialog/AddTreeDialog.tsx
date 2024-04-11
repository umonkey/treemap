/**
 * MUI component reference:
 * https://mui.com/material-ui/all-components/
 */

import { useState } from "react";
import { Box, Button, FormHelperText, TextField } from "@mui/material";

import { IAddTreeRequest, ILatLng } from "@/types";
import "./styles.css";

interface IProps {
  center: ILatLng | null;
  error: string | null;
  busy: boolean;
  onSave: (tree: IAddTreeRequest) => void;
  onCancel?: () => void;
}

export const AddTreeDialog = (props: IProps) => {
  const [species, setSpecies] = useState<string>('');
  const [height] = useState<number|undefined>(undefined);
  const [circumference] = useState<number|undefined>(undefined);
  const [diameter] = useState<number|undefined>(undefined);

  const isSaveEnabled = (): boolean => {
    if (species.length === 0) {
      return false;
    }

    if (props.busy) {
      return false;
    }

    if (props.center === null) {
      return false;
    }

    return true;
  };

  const handleSpeciesChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSpecies(event.target.value);
  };

  const handleSaveClick = async () => {
    if (!props.center) {
      console.error("Center is not set, cannot add tree.");
      return;
    }

    if (!species) {
      console.error("Species not set, cannot add tree.");
      return;
    }

    props.onSave({
        lat: props.center.lat,
        lon: props.center.lon,
        name: species,
        height: height || null,
        circumference: circumference || null,
        diameter: diameter || null,
    } as IAddTreeRequest);
  };

  const handleCancelClick = () => {
    props.onCancel && props.onCancel();
  };

  return (
    <div className="AddTreeDialog Dialog">
      <Box component="form">
        <h2>Describe the tree</h2>

        <div className="group">
          <TextField id="species" label="Species" variant="standard" required value={species} onChange={handleSpeciesChange} />
          <FormHelperText>Enter English or Latin name.</FormHelperText>
        </div>

        <div className="group">
          <TextField id="height" label="Height, m" variant="standard" type="number" value={height} />
        </div>

        <div className="group">
          <TextField id="circumference" label="Circumference, m" variant="standard" type="number" value={circumference} />
        </div>

        <div className="group">
          <TextField id="diameter" label="Diameter, m" variant="standard" type="number" value={diameter} />
        </div>

        <div className="group">
          <Button variant="contained" color="success" disabled={!isSaveEnabled()} onClick={handleSaveClick}>Confirm</Button>
          <Button color="secondary" onClick={handleCancelClick}>Cancel</Button>
        </div>
      </Box>
    </div>
  );
}
