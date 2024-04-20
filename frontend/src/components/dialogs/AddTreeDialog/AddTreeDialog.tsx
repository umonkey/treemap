/**
 * MUI component reference:
 * https://mui.com/material-ui/all-components/
 */

import { useState } from "react";
import { Box, Button, ButtonGroup, FormHelperText, TextField } from "@mui/material";

import { SpeciesSelector, TreeStateSelector } from "@/components";
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
  const [notes, setNotes] = useState<string>("");

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

  const handleNotesChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setNotes(event.target.value);
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
      notes: notes || null,
    } as IAddTreeRequest);
  };

  const handleCancelClick = () => {
    props.onCancel && props.onCancel();
  };

  const handleNameChange = (value: string) => {
    setName(value);
  };

  return (
    <div className="AddTreeDialog Dialog">
      <Box component="form">
        <h2>Describe the tree</h2>

        <div className="group wide">
          <SpeciesSelector onChange={handleNameChange} />
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

        <div className="group wide">
          <TextField id="notes" label="Notes" variant="standard" value={notes} onChange={handleNotesChange} />
          <FormHelperText>Add for notable trees, like: Queen's Oak.</FormHelperText>
        </div>

        {props.error && (
          <p className="error">{props.error}</p>
        )}

        <div className="group">
          <ButtonGroup variant="contained">
            <Button color="success" disabled={!isSaveEnabled()} onClick={handleSaveClick}>Confirm</Button>
            <Button onClick={handleCancelClick}>Cancel</Button>
          </ButtonGroup>
        </div>
      </Box>
    </div>
  );
}
