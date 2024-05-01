import { useParams } from "react-router-dom";

export const useDetailsPageWrapper = () => {
  const { id } = useParams();

  if (!id) {
    throw Error("Missing tree id.");
  }

  return {
    id,
  };
};
