// Global imports.
import IconButton from "@mui/material/IconButton";
import HomeIcon from "@mui/icons-material/Home";

// Local imports.
import { useHomeButton } from "./hooks";

export const HomeButton = () => {
  const { handleClick } = useHomeButton();

  return (
    <IconButton
      size="large"
      edge="start"
      color="inherit"
      aria-label="home"
      sx={{ mr: 2 }}
      onClick={handleClick}
    >
      <HomeIcon />
    </IconButton>
  );
}
