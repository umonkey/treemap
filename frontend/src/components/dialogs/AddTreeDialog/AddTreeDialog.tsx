/**
 * MUI component reference:
 * https://mui.com/material-ui/all-components/
 */

// Global imports.
import { Box, FormHelperText, TextField } from "@mui/material";

// Project imports.
import { ConfirmCancelButtons, SpeciesSelector, TreeStateSelector } from "@/components";

// Local imports.
import { useAddTreeDialog } from "./hooks";
import { IProps } from "./types";
import "./styles.scss";

export const AddTreeDialog = (props: IProps) => {
  const {
    canSave,
    circumference,
    diameter,
    error,
    handleCancelClick,
    handleCircumferenceChange,
    handleDiameterChange,
    handleHeightChange,
    handleNameChange,
    handleNotesChange,
    handleSaveClick,
    handleStateChange,
    height,
    notes,
    species,
    state,
  } = useAddTreeDialog(props);

  return (
    <div className="AddTreeDialog Dialog">
      <Box component="form">
        <div className="group wide">
          <SpeciesSelector value={species} onChange={handleNameChange} />
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
            <TreeStateSelector state={state} onChange={handleStateChange} />
          </div>
        </div>

        <div className="group wide">
          <TextField id="notes" label="Notes" variant="standard" value={notes} onChange={handleNotesChange} />
          <FormHelperText>Add for notable trees, like: Queen's Oak.</FormHelperText>
        </div>

        {error && (
          <p className="error">{error}</p>
        )}

        <ConfirmCancelButtons
          canConfirm={canSave}
          onCancel={handleCancelClick}
          onConfirm={handleSaveClick}
        />
      </Box>
    </div>
  );
}
