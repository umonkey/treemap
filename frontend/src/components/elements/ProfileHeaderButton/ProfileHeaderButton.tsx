import { IconButton } from "@mui/material";
import AccountCircle from "@mui/icons-material/AccountCircle";
import LogoutIcon from '@mui/icons-material/Logout';

import { useProfileHeaderButton } from "./hooks";

export const ProfileHeaderButton = () => {
  const { isLoggedIn, loginFunction, logoutFunction } = useProfileHeaderButton();

  if (isLoggedIn) {
    return (
      <IconButton
        size="large"
        edge="end"
        aria-label="account of current user"
        aria-haspopup="true"
        color="inherit"
        onClick={logoutFunction}
      >
        <LogoutIcon />
      </IconButton>
    );
  }

  return (
    <IconButton
      size="large"
      edge="end"
      aria-label="account of current user"
      aria-haspopup="true"
      color="inherit"
      onClick={loginFunction}
    >
      <AccountCircle />
    </IconButton>
  );
};
