import { NarrowPage, TreeListItem, PrevNextNavigation } from "@/components";
import { locale } from "@/locale";

import { useNewTreesPage } from "./hooks";
import { IProps } from "./types";
import "./styles.scss";

export const NewTreesPage = (props: IProps) => {
  const { error, loading, trees, prev, next } = useNewTreesPage(props);

  return (
    <NarrowPage className="NewTreesPage">
      <h1>{locale.newTreesHeader()}</h1>

      {loading && !trees && (
        <p>{locale.loading()}</p>
      )}

      {error && (
        <p>{error}</p>
      )}

      {trees && !error && (
        <>
          <div className="items">
            {trees.map((tree, index) => (
              <TreeListItem
                key={index}
                tree={tree}
              />
            ))}
          </div>

          <PrevNextNavigation
            prev={prev}
            next={next}
          />
        </>
      )}
    </NarrowPage>
  );
}
