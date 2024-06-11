// Global imports.
import { useEffect, useState } from "react";
import { Box, FormHelperText, TextField } from "@mui/material";

// Project imports.
import { ConfirmCancelButtons, SpeciesSelector, TreeStateSelector } from "@/components";
import { ITreeDetails } from "@/types";
import { locale } from "@/locale";

// Local imports.
import "./styles.scss";

interface IProps {
  tree: ITreeDetails;
  busy: boolean;
  error?: string | null;
  onSave: (tree: ITreeDetails) => void;
  onCancel?: () => void;
}

export const EditTreeDialog = (props: IProps) => {
  const [species, setSpecies] = useState<string>(props.tree.species);
  const [height, setHeight] = useState<number>(props.tree.height || 0.0);
  const [circumference, setCircumference] = useState<number>(props.tree.circumference || 0.0);
  const [diameter, setDiameter] = useState<number>(props.tree.diameter || 0.0);
  const [state, setState] = useState<string>(props.tree.state || "healthy");
  const [notes, setNotes] = useState<string>(props.tree.notes || "");

  useEffect(() => {
    setSpecies(props.tree.species);
    setHeight(props.tree.height || 0.0);
    setCircumference(props.tree.circumference || 0.0);
    setDiameter(props.tree.diameter || 0.0);
  }, [props.tree]);

  const handleNameChange = (value: string) => {
    setSpecies(value);
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

  const handleNotesChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setNotes(event.target.value);
  };

  const handleSaveClick = async () => {
    if (!species) {
      console.error("species not set, cannot add tree.");
      return;
    }

    props.onSave({
      id: props.tree.id,
      lat: props.tree.lat,
      lon: props.tree.lon,
      species,
      notes,
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
    if (!species) {
      return false;
    }

    if (props.busy) {
      return false;
    }

    return true;
  };

  return (
    <div className="EditTreeDialog">
      <Box component="form">
        <div className="group wide">
          <SpeciesSelector value={species} onChange={handleNameChange} />
        </div>

        <div className="row">
          <div className="group short">
            <TextField id="height" label={locale.heightLabel()} variant="standard" type="number" value={height} onChange={handleHeightChange} />
          </div>

          <div className="group short">
            <TextField id="circumference" label={locale.circumferenceLabel()} variant="standard" type="number" value={circumference} onChange={handleCircumferenceChange} />
          </div>
        </div>

        <div className="row">
          <div className="group short">
            <TextField id="diameter" label={locale.canopyLabel()} variant="standard" type="number" value={diameter} onChange={handleDiameterChange} />
          </div>

          <div className="group short">
            <TreeStateSelector state={state} onChange={handleStateChange} />
          </div>
        </div>

        <div className="group wide">
          <TextField id="notes" label={locale.notes()} variant="standard" value={notes} onChange={handleNotesChange} />
          <FormHelperText>{locale.notesHint()}</FormHelperText>
        </div>

        {props.error && (
          <p className="error">{props.error}</p>
        )}

        <ConfirmCancelButtons
          onConfirm={handleSaveClick}
          onCancel={handleCancelClick}
          canConfirm={canSave()}
        />
      </Box>
    </div>
  );
};
