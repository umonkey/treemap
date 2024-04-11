import { Button } from "@mui/material";
import { ILatLng } from "@/types";

import "./styles.scss";

interface IProps {
  position: ILatLng | null;
  onContinue: () => void;
  onCancel: () => void;
}

export const SelectLocationDialog = (props: IProps) => {
  const handleContinue = () => {
    props.onContinue();
  };

  const handleCancel = () => {
    props.onCancel();
  };

  const canContinue = !!props.position;

  return (
    <div className="SelectLocationDialog Dialog">
      <h2>Adding a new tree</h2>
      <p>Move the map to select tree location.&nbsp; You can correct it later.</p>

      <div className="buttons">
        <Button variant="contained" color="success" disabled={!canContinue} onClick={handleContinue}>Continue</Button>
        <Button color="secondary" onClick={handleCancel}>Cancel</Button>
      </div>
    </div>
  );
};
