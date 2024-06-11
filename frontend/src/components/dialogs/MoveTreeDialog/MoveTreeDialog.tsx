import { ILatLng } from "@/types";

import { ConfirmCancelButtons } from "@/components";
import { locale } from "@/locale";

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

  return (
    <div className="MoveTreeDialog Dialog">
      <h2>{locale.moveTitle()}</h2>
      <p>{locale.moveDescription()}</p>

      {props.error && (
        <p className="error">{props.error}</p>
      )}

      <ConfirmCancelButtons
        onConfirm={handleContinue}
        onCancel={handleCancel}
        canConfirm={canContinue}
      />
    </div>
  );
};
