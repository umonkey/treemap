// Global imports.
import { Button } from "@mui/material";

// Local imports.
import "./styles.scss";

interface IProps {
  canConfirm: boolean;
  onCancel: () => void;
  onConfirm: () => void;
}

export const ConfirmCancelButtons = (props: IProps) => {
  return (
    <div className="ConfirmCancelButtons">
      <Button onClick={props.onCancel}>Cancel</Button>
      <Button variant="contained" disabled={!props.canConfirm} onClick={props.onConfirm}>Confirm</Button>
    </div>
  );
};
