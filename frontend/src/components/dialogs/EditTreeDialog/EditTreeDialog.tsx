import { useEffect, useState } from "react";
import { Box, Button, FormHelperText, TextField } from "@mui/material";

import { TreeStateSelector } from "@/components";
import { ITreeDetails } from "@/types";
import "./styles.scss";

interface IProps {
  tree: ITreeDetails;
  busy: boolean;
  error?: string | null;
  onSave: (tree: ITreeDetails) => void;
  onCancel?: () => void;
}

export const EditTreeDialog = (props: IProps) => {
  const [name, setName] = useState<string>('');
  const [height, setHeight] = useState<number>(0.0);
  const [circumference, setCircumference] = useState<number>(0.0);
  const [diameter, setDiameter] = useState<number>(0.0);
  const [state, setState] = useState<string>(props.tree.state || "healthy");

  useEffect(() => {
    setName(props.tree.name);
    setHeight(props.tree.height || 0.0);
    setCircumference(props.tree.circumference || 0.0);
    setDiameter(props.tree.diameter || 0.0);
  }, [props.tree]);

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

  const handleStateChange = (value: string) => {
    setState(value);
  };

  const handleSaveClick = async () => {
    if (!name) {
      console.error("Name not set, cannot add tree.");
      return;
    }

    props.onSave({
      id: props.tree.id,
      lat: props.tree.lat,
      lon: props.tree.lon,
      name,
      height,
      circumference,
      diameter,
      state,
    } as ITreeDetails);
  };

  const handleCancelClick = () => {
    props.onCancel && props.onCancel();
  };

  const canSave = (): boolean => {
    if (!name) {
      return false;
    }

    if (props.busy) {
      return false;
    }

    return true;
  };

  return (
    <div className="EditTreeDialog">
      <h2>Update tree details</h2>

      <Box component="form">
        <div className="group wide">
          <TextField id="name" label="Name" variant="standard" required value={name} onChange={handleNameChange} />
          <FormHelperText>Enter English or Latin name.</FormHelperText>
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

          <div className="group short">
            <TreeStateSelector state={state} onChange={handleStateChange} />
          </div>
        </div>

        {props.error && (
          <p className="error">{props.error}</p>
        )}

        <div className="group buttons">
          <Button variant="contained" color="success" disabled={!canSave()} onClick={handleSaveClick}>Confirm</Button>
          <Button color="secondary" onClick={handleCancelClick}>Cancel</Button>
        </div>
      </Box>
    </div>
  );
};
