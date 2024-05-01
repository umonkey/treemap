// Project imports.
import { WithAuth } from "@/components";
import { AddTreeDetailsPage } from "@/pages";

// Local imports.
import { useAddTreeDetailsPageWrapper } from "./hooks";

export const AddTreeDetailsPageWrapper = () => {
  const { lat, lon } = useAddTreeDetailsPageWrapper();

  return (
    <WithAuth>
      <AddTreeDetailsPage lat={lat} lon={lon} />
    </WithAuth>
  );
};
