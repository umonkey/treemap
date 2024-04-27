import { IconButton } from "@mui/material";
import AccountCircle from "@mui/icons-material/AccountCircle";

import { useUserInfo } from "@/hooks";

export const ProfileHeaderButton = () => {
  const { userInfo } = useUserInfo();

  if (userInfo) {
    return (
      <IconButton
        size="large"
        edge="end"
        aria-label="account of current user"
        aria-haspopup="true"
        color="inherit"
      >
        <AccountCircle />
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
    >
      <AccountCircle />
    </IconButton>
  );
};
