import { useEffect, useState } from "react";
import { Box, Button, FormHelperText, TextField } from "@mui/material";

import { treeMapService } from "@/services/api";
import { ILatLng, ITreeInfo } from "@/types";
import "./styles.css";

interface IProps {
  center: ILatLng;
  onSuccess: (tree: ITreeInfo) => void;
}

export const AddTreeDialog = (props: IProps) => {
  const [species, setSpecies] = useState<string>('');
  const [sending, setSending] = useState<boolean>(false);
  const [saveEnabled, setSaveEnabled] = useState<boolean>(false);

  useEffect(() => {
    setSaveEnabled((species.length > 0) && !sending);
  }, [sending, species]);

  const handleSpeciesChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSpecies(event.target.value);
  };

  const handleSaveClick = async () => {
    try {
      setSending(true);

      const tree = await treeMapService.addMarker({
        lat: props.center.lat,
        lon: props.center.lon,
        species: species,
      });

      console.debug(`Tree added with id ${tree.id}`);
      props.onSuccess(tree);
    } finally {
      setSending(false);
    }
  };

  return (
    <Box component="form" className="AddTreeDialog">
      <div className="group">
        <TextField id="species" label="Species" variant="standard" required value={species} onChange={handleSpeciesChange} />
        <FormHelperText>Enter English or Latin name.</FormHelperText>
      </div>

      <div className="group">
        <TextField id="coords" label="Coordinates" variant="standard" disabled value={`${props.center.lat}, ${props.center.lon}`} />
      </div>

      <div className="group">
        <Button variant="contained" color="success" disabled={!saveEnabled} onClick={handleSaveClick}>Save</Button>
      </div>
    </Box>
  );
}
