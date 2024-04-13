/**
 * MUI component reference:
 * https://mui.com/material-ui/all-components/
 */

import { useState } from "react";
import { Box, Button, FormHelperText, TextField } from "@mui/material";

import { TreeStateSelector } from "@/components";
import { IAddTreeRequest, ILatLng } from "@/types";
import "./styles.scss";

interface IProps {
  center: ILatLng | null;
  error: string | null;
  busy: boolean;
  onSave: (tree: IAddTreeRequest) => void;
  onCancel?: () => void;
}

export const AddTreeDialog = (props: IProps) => {
  const [name, setName] = useState<string>('');
  const [height, setHeight] = useState<number|undefined>(undefined);
  const [circumference, setCircumference] = useState<number|undefined>(undefined);
  const [diameter, setDiameter] = useState<number|undefined>(undefined);
  const [state, setState] = useState<string>('healthy');

  const isSaveEnabled = (): boolean => {
    if (name.length === 0) {
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

  const handleNameChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setName(event.target.value);
  };

  const handleHeightChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setHeight(parseFloat(event.target.value));
  };

  const handleCircumferenceChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setCircumference(parseFloat(event.target.value));
  };

  const handleDiameterChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setDiameter(parseFloat(event.target.value));
  };

  const handleSaveClick = async () => {
    if (!props.center) {
      console.error("Center is not set, cannot add tree.");
      return;
    }

    if (!name) {
      console.error("Name not set, cannot add tree.");
      return;
    }

    props.onSave({
      lat: props.center.lat,
      lon: props.center.lon,
      name: name,
      height: height || 0,
      circumference: circumference || 0,
      diameter: diameter || 0,
      state,
    } as IAddTreeRequest);
  };

  const handleCancelClick = () => {
    props.onCancel && props.onCancel();
  };

  return (
    <div className="AddTreeDialog Dialog">
      <Box component="form">
        <h2>Describe the tree</h2>

        <div className="group wide">
          <TextField id="name" label="Name" variant="standard" required value={name} onChange={handleNameChange} />
          <FormHelperText>Describe the tree.</FormHelperText>
        </div>

        <div className="row">
          <div className="group short">
            <TextField id="height" label="Height, m" variant="standard" type="number" value={height} onChange={handleHeightChange} />
          </div>

          <div className="group short">
            <TextField id="circumference" label="Circumference, m" variant="standard" type="number" value={circumference} onChange={handleCircumferenceChange} />
          </div>
        </div>

        <div className="row">
          <div className="group short">
            <TextField id="diameter" label="Canopy ⌀, m" variant="standard" type="number" value={diameter} onChange={handleDiameterChange} />
          </div>

          <div className="short">
            <TreeStateSelector
              state={state}
              onChange={(value: string) => setState(value)}
            />
          </div>
        </div>

        {props.error && (
          <p className="error">{props.error}</p>
        )}

        <div className="group">
          <Button variant="contained" color="success" disabled={!isSaveEnabled()} onClick={handleSaveClick}>Confirm</Button>
          <Button color="secondary" onClick={handleCancelClick}>Cancel</Button>
        </div>
      </Box>
    </div>
  );
}
