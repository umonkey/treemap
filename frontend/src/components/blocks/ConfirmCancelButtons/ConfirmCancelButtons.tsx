// Global imports.
import { Button } from "@mui/material";

// Project imports.
import { locale } from "@/locale";

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
      <Button onClick={props.onCancel}>{locale.cancel()}</Button>
      <Button variant="contained" disabled={!props.canConfirm} onClick={props.onConfirm}>{locale.confirm()}</Button>
    </div>
  );
};
