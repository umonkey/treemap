import {
  AddTreeDialog,
  NarrowPage,
  PositionSelector,
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
    <NarrowPage className="AddPage">
      <h1>Adding new trees</h1>

      <PositionSelector
        onChange={handlePointsChange}
      />

      <AddTreeDialog points={points} />
    </NarrowPage>
  );
};
