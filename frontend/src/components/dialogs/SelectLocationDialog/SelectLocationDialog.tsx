import { Button } from "@mui/material";
import { ILatLng } from "@/types";

import { useGoogleAuth } from "./hooks";

interface IProps {
  position: ILatLng | null;
  onContinue: () => void;
}

export const SelectLocationDialog = (props: IProps) => {
  const { profile, login } = useGoogleAuth();

  const handleContinueClick = () => {
    console.debug("Continue clicked.");
    props.onContinue();
  };

  const canContinue = !!props.position;

  return (
    <>
      <h2>Adding a new tree</h2>

      {profile && (
        <>
          <p>Please click the map to set the location of the tree you are going to add.</p>
          <p>You can drag the marker around to fine-tune the location.</p>

          <Button variant="contained" color="success" disabled={!canContinue} onClick={handleContinueClick}>Continue</Button>
        </>
      )}

      {!profile && (
        <>
          <p>You need to log in first.</p>
          <Button variant="contained" color="success" onClick={login}>Log in with Google</Button>
        </>
      )}
    </>
  );
};
