import { WithAuth } from "@/components";
import { EditTreePage } from "@/pages";

import { useEditTreePageWrapper } from "./hooks";

export const EditTreePageWrapper = () => {
  const { id, handleSuccess, handleCancel } = useEditTreePageWrapper();

  return (
    <WithAuth>
      <EditTreePage
        id={id}
        onSuccess={handleSuccess}
        onCancel={handleCancel}
      />
    </WithAuth>
  );
};
