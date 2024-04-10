import { Button } from "@mui/material";
import { ILatLng } from "@/types";

interface IProps {
  position: ILatLng | null;
  onContinue: () => void;
}

export const SelectLocationDialog = (props: IProps) => {
  const handleContinueClick = () => {
    console.debug("Continue clicked.");
    props.onContinue();
  };

  const canContinue = !!props.position;

  return (
    <>
      <h2>Adding a new tree</h2>
      <p>Please move the map to select tree location.</p>
      <Button variant="contained" color="success" disabled={!canContinue} onClick={handleContinueClick}>Continue</Button>
    </>
  );
};
