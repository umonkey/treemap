import { Button } from "@mui/material";
import { ILatLng } from "@/types";

import "./styles.scss";

interface IProps {
  position: ILatLng | null;
  busy: boolean;
  error?: string | null;
  onContinue: () => void;
  onCancel: () => void;
}

export const MoveTreeDialog = (props: IProps) => {
  const handleContinue = () => {
    props.onContinue();
  };

  const handleCancel = () => {
    props.onCancel();
  };

  const canContinue = !!props.position && !props.busy;

  const label = props.error ? "Retry" : "Confirm";

  return (
    <div className="MoveTreeDialog Dialog">
      <h2>Select new tree location</h2>
      <p>Move the map to select tree correct location for the tree.</p>

      {props.error && (
        <p className="error">{props.error}</p>
      )}

      <div className="buttons">
        <Button variant="contained" color="success" disabled={!canContinue} onClick={handleContinue}>{label}</Button>
        <Button color="secondary" onClick={handleCancel}>Cancel</Button>
      </div>
    </div>
  );
};
