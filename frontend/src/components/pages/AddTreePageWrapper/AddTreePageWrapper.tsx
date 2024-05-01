// Project imports.
import { AddTreePage, WithAuth } from "@/components";

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
