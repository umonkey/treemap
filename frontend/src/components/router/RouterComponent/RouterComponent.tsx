import { Route, Routes } from "react-router-dom";
import { routes } from "@/utils/routes";

import { AddTreePageWrapper, HomePage } from "@/components";

export const RouterComponent = () => {
  return (
    <Routes>
      <Route path={routes.home()} element={<HomePage />} />
      <Route path="/add" element={<AddTreePageWrapper />} />
    </Routes>
  );
};
