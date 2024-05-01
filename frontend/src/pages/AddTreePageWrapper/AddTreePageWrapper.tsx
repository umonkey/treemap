// Project imports.
import { WithAuth } from "@/components";
import { AddTreePage } from "@/pages";

// Local imports.
import { useAddTreePageWrapper } from "./hooks";

export const AddTreePageWrapper = () => {
  const { lat, lon } = useAddTreePageWrapper();

  return (
    <WithAuth>
      <AddTreePage lat={lat} lon={lon} />
    </WithAuth>
  );
};
