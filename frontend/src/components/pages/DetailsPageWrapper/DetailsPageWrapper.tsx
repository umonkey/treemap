import { useParams } from "react-router-dom";

import { DetailsPage } from "@/components";

export const DetailsPageWrapper = () => {
  const { id } = useParams();

  if (!id) {
    console.error("Missing tree id.");
    return null;
  }

  return (
    <DetailsPage id={id} />
  );
};
