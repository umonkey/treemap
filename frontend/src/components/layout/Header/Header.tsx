import { AppBar, Box, Toolbar, Typography } from "@mui/material";

import { ProfileHeaderButton, SearchBar } from "@/components";

import { useHeader } from "./hooks";
import "./styles.scss";

export const Header = () => {
  const { searchQuery, handleSearch } = useHeader();

  return (
    <div className="Header">
      <AppBar>
        <Toolbar variant="dense">
          <Typography variant="h6" color="inherit" noWrap>Tree Map</Typography>

          <Box>
            <SearchBar
              searchQuery={searchQuery}
              onChange={handleSearch}
            />
          </Box>

          <Box sx={{ flexGrow: 1 }} />

          <Box>
            <ProfileHeaderButton />
          </Box>
        </Toolbar>
      </AppBar>
    </div>
  );
};
