import { ILatLng } from "@/types";

import { ConfirmCancelButtons } from "@/components";

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
      <h2>Where to add the tree?</h2>
      <p>Move the map to select tree location.&nbsp; You can correct it later.</p>

      <ConfirmCancelButtons
        onConfirm={handleContinue}
        onCancel={handleCancel}
        canConfirm={canContinue}
      />
    </div>
  );
};
