// Global imports.
import { renderHook } from "@testing-library/react";

// Project imports.
import { ITreeInfo } from "@/types";
import { SAMPLE_TREES } from "@/sample-data";

// Local imports.
import { useExternalTreeLinks } from "./hooks";

describe("ExternalTreeLinks hooks", () => {
  test("Render OSM link", () => {
    const tree = {
      ...SAMPLE_TREES[0],
      osm_id: 12345,
    } as ITreeInfo;

    const links = renderHook(() => useExternalTreeLinks(tree)).result.current.links;

    expect(links.length).toEqual(2);

    expect(links[0].text).toEqual("OSM");
    expect(links[0].href).toEqual("https://www.openstreetmap.org/node/12345");
    expect(links[0].tooltip).toEqual("Open this tree on OSM");
  });

  test("Render Wikidata link", () => {
    const tree = {
      ...SAMPLE_TREES[0],
      species: "Acer",
    } as ITreeInfo;

    const links = renderHook(() => useExternalTreeLinks(tree)).result.current.links;

    expect(links.length).toEqual(2);

    expect(links[1].text).toEqual("Wikidata");
    expect(links[1].href).toEqual("https://www.wikidata.org/w/index.php?search=Acer");
    expect(links[1].tooltip).toEqual("Search for this tree on Wikidata");
  });
});
