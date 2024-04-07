import { Button } from "@mui/material";
import { ILatLng } from "@/types";

import { LoginWithGoogleButton } from "@/components";
import { useUserInfo } from "@/utils/userinfo";

interface IProps {
  position: ILatLng | null;
  onContinue: () => void;
}

export const SelectLocationDialog = (props: IProps) => {
  const { userInfo } = useUserInfo();

  const handleContinueClick = () => {
    console.debug("Continue clicked.");
    props.onContinue();
  };

  const handleLoginSuccess = () => {
    console.debug("Login successful.");

    // For some reason, userInfo is not updated immediately after login,
    // so we need to reload the page to get the updated userInfo.
    window.location.reload();
  };

  const handleLoginError = () => {
    console.debug("Login failed.");
  };

  console.debug("userInfo:", userInfo);

  const canContinue = !!props.position;

  return (
    <>
      <h2>Adding a new tree</h2>

      {userInfo && (
        <>
          <p>Please click the map to set the location of the tree you are going to add.</p>
          <p>You can drag the marker around to fine-tune the location.</p>

          <Button variant="contained" color="success" disabled={!canContinue} onClick={handleContinueClick}>Continue</Button>
        </>
      )}

      {!userInfo && (
        <>
          <p>You need to log in first.</p>

          <LoginWithGoogleButton
            onSuccess={handleLoginSuccess}
            onError={handleLoginError}
          />
        </>
      )}
    </>
  );
};
