import { useParams } from "react-router-dom";

import { PreviewPage } from "@/pages";

export const PreviewPageWrapper = () => {
  const { id } = useParams();

  if (!id) {
    console.error("Missing tree id.");
    return null;
  }

  return (
    <PreviewPage id={id} />
  );
};
