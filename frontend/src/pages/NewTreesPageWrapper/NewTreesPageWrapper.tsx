import { useSearchParams } from "react-router-dom";

import { NewTreesPage } from "@/pages";

export const NewTreesPageWrapper = () => {
  const [params] = useSearchParams();

  return (
    <NewTreesPage
      count={parseInt(params.get("count") || "50")}
      skip={parseInt(params.get("skip") || "0")}
    />
  );
};
