import { Route, Routes } from "react-router-dom";
import { routes } from "@/utils/routes";

import {
  AddPage,
  DetailsPageWrapper,
  EditTreePageWrapper,
  HomePage,
  MoveTreePageWrapper,
  NewTreesPageWrapper,
  PreviewPageWrapper,
} from "@/pages";

export const RouterComponent = () => {
  return (
    <Routes>
      <Route path={routes.home()} element={<HomePage />} />
      <Route path={routes.add()} element={<AddPage />} />
      <Route path={routes.treeDetails(":id")} element={<DetailsPageWrapper />} />
      <Route path={routes.editTree(":id")} element={<EditTreePageWrapper />} />
      <Route path={routes.moveTree(":id")} element={<MoveTreePageWrapper />} />
      <Route path={routes.treePreview(":id")} element={<PreviewPageWrapper />} />
      <Route path={routes.newTrees()} element={<NewTreesPageWrapper />} />
    </Routes>
  );
};
