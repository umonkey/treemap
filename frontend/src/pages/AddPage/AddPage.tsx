import {
  AddTreeDialog,
  NarrowPage,
  PositionSelector,
  WithAuth,
} from "@/components";
import { locale } from "@/locale";

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
        <h1>{locale.addTitle()}</h1>

        <PositionSelector
          onChange={handlePointsChange}
        />

        <AddTreeDialog points={points} />
      </NarrowPage>
    </WithAuth>
  );
};
