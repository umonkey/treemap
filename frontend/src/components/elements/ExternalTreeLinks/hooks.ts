// Global imports.
import { useMemo } from "react";

// Project imports.
import { ITreeInfo } from "@/types";

interface ILink {
  href: string;
  tooltip: string;
  text: string;
}

export const useExternalTreeLinks = (tree: ITreeInfo) => {
  const links = useMemo(() => {
    const items = [] as ILink[];

    if (tree.osm_id) {
      items.push({
        href: `https://www.openstreetmap.org/node/${tree.osm_id}`,
        tooltip: "Open this tree on OSM",
        text: "OSM",
      } as ILink);
    }

    items.push({
      href: `https://www.wikidata.org/w/index.php?search=${tree.species}`,
      tooltip: "Search for this tree on Wikidata",
      text: "Wikidata",
    } as ILink);

    return items;
  }, [tree]);

  return {
    links,
  };
};
