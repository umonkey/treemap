import { NarrowPage, TreeListItem } from "@/components";
import { locale } from "@/locale";

import { useNewTreesPage } from "./hooks";
import { IProps } from "./types";
import "./styles.scss";

export const NewTreesPage = (props: IProps) => {
  const { error, loading, trees } = useNewTreesPage(props);

  return (
    <NarrowPage className="NewTreesPage">
      <h1>Recently Added Trees</h1>

      {loading && !trees && (
        <p>{locale.loading()}</p>
      )}

      {error && (
        <p>{error}</p>
      )}

      {trees && !error && (
        <div className="items">
          {trees.map((tree, index) => (
            <TreeListItem
              key={index}
              tree={tree}
            />
          ))}
        </div>
      )}
    </NarrowPage>
  );
}
