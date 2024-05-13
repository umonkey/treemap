import {
  AddTreeDialog,
  NarrowPage,
  PositionSelector,
  WithAuth,
} from "@/components";

// Local imports.
import { useAddPage } from "./hooks";
import "./styles.scss";

export const AddPage = () => {
  const {
    handlePointsChange,
    points,
  } = useAddPage();

  return (
    <WithAuth>
      <NarrowPage className="AddPage">
        <h1>Adding new trees</h1>

        <PositionSelector
          onChange={handlePointsChange}
        />

        <AddTreeDialog points={points} />
      </NarrowPage>
    </WithAuth>
  );
};
