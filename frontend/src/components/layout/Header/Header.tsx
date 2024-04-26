import { AppBar, Box, Toolbar, Typography, IconButton } from "@mui/material";
import MenuIcon from "@mui/icons-material/Menu";
import AccountCircle from "@mui/icons-material/AccountCircle";

import { SearchBar } from "@/components";

import { useHeader } from "./hooks";
import "./styles.scss";

export const Header = () => {
  const { searchQuery, handleSearch } = useHeader();

  return (
    <div className="Header">
      <AppBar>
        <Toolbar variant="dense">
          <IconButton edge="start" color="inherit" aria-label="menu">
            <MenuIcon />
          </IconButton>

          <Typography variant="h6" color="inherit" noWrap>Tree Map</Typography>

          <Box>
            <SearchBar
              searchQuery={searchQuery}
              onChange={handleSearch}
            />
          </Box>

          <Box sx={{ flexGrow: 1 }} />

          <Box>
            <IconButton
              size="large"
              edge="end"
              aria-label="account of current user"
              aria-haspopup="true"
              color="inherit"
            >
              <AccountCircle />
            </IconButton>
          </Box>
        </Toolbar>
      </AppBar>
    </div>
  );
};
