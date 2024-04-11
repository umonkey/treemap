import { useEffect, useState } from "react";
import { Box, Button, FormHelperText, TextField } from "@mui/material";

import { ITreeDetails } from "@/types";
import "./styles.scss";

interface IProps {
  tree: ITreeDetails;
  onSave: (tree: ITreeDetails) => void;
  onCancel?: () => void;
}

export const EditTreeDialog = (props: IProps) => {
  const [name, setName] = useState<string>('');
  const [height, setHeight] = useState<number>(0.0);
  const [circumference, setCircumference] = useState<number>(0.0);
  const [diameter, setDiameter] = useState<number>(0.0);
  const [saveEnabled, setSaveEnabled] = useState<boolean>(false);

  useEffect(() => {
    setName(props.tree.name);
    setHeight(props.tree.height || 0.0);
    setCircumference(props.tree.circumference || 0.0);
    setDiameter(props.tree.diameter || 0.0);
  }, [props.tree]);

  useEffect(() => {
    setSaveEnabled(!!name);
  }, [name]);

  const handleNameChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setName(event.target.value);
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
    } as ITreeDetails);
  };

  const handleCancelClick = () => {
    props.onCancel && props.onCancel();
  };

  return (
    <div className="EditTreeDialog">
      <Box component="form">
        <div className="group">
          <TextField id="name" label="Name" variant="standard" required value={name} onChange={handleNameChange} />
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
          <Button variant="contained" color="success" disabled={!saveEnabled} onClick={handleSaveClick}>Confirm</Button>
          <Button color="secondary" onClick={handleCancelClick}>Cancel</Button>
        </div>
      </Box>
    </div>
  );
};
