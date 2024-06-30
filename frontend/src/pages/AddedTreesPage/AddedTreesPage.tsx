import { NarrowPage } from "@/components";
import { locale } from "@/locale";

import { useAddedTreesPage } from "./hooks";
import { IProps } from "./types";

export const AddedTreesPage = (props: IProps) => {
  const { error, loading, trees } = useAddedTreesPage(props);

  return (
    <NarrowPage className="AddedTreesPage">
      <h1>Recently Added Trees</h1>

      {loading && !trees && (
        <p>{locale.loading()}</p>
      )}

      {error && (
        <p>{error}</p>
      )}

      {trees && !error && (
        <ul>
          {trees.map((tree, index) => (
            <li key={index}>{tree.id}</li>
          ))}
        </ul>
      )}
    </NarrowPage>
  );
}
