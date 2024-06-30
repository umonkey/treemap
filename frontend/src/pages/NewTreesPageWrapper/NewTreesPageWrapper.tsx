import { useParams } from "react-router-dom";

import { NewTreesPage } from "@/pages";

export const NewTreesPageWrapper = () => {
  const { count, skip } = useParams();

  return (
    <NewTreesPage
      count={parseInt(count || "50")}
      skip={parseInt(skip || "0")}
    />
  );
};
