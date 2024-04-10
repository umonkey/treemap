/**
 * MUI component reference:
 * https://mui.com/material-ui/all-components/
 */

import { useEffect, useState } from "react";
import { Box, Button, FormHelperText, TextField } from "@mui/material";

import { treeMapService } from "@/services/api";
import { ILatLng, ITreeInfo } from "@/types";
import "./styles.css";

interface IProps {
  center: ILatLng | null;
  onSuccess: (tree: ITreeInfo) => void;
  onCancel?: () => void;
}

export const AddTreeDialog = (props: IProps) => {
  const [species, setSpecies] = useState<string>('');
  const [height] = useState<number|undefined>(undefined);
  const [circumference] = useState<number|undefined>(undefined);
  const [diameter] = useState<number|undefined>(undefined);

  const [sending, setSending] = useState<boolean>(false);
  const [saveEnabled, setSaveEnabled] = useState<boolean>(false);

  useEffect(() => {
    setSaveEnabled((species.length > 0) && !sending && props.center !== null);
  }, [sending, species, props.center]);

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

    try {
      setSending(true);

      const tree = await treeMapService.addMarker({
        lat: props.center.lat,
        lon: props.center.lon,
        species: species,
        height: height || null,
        circumference: circumference || null,
        diameter: diameter || null,
      });

      console.debug("Tree added.", tree);
      props.onSuccess(tree);
    } finally {
      setSending(false);
    }
  };

  const handleCancelClick = () => {
    props.onCancel && props.onCancel();
  };

  return (
    <Box component="form" className="AddTreeDialog">
      <h2>Add a tree</h2>

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
        <Button variant="contained" color="success" disabled={!saveEnabled} onClick={handleSaveClick}>Confirm</Button>
        <Button color="secondary" onClick={handleCancelClick}>Cancel</Button>
      </div>
    </Box>
  );
}
