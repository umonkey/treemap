import type { Preview } from "@storybook/react";
import { MemoryRouter } from "react-router-dom";

import { StoryWrapper } from "./StoryWrapper";
import { ITreeInfo, ITreeDetails } from "@/types";

import "./preview.css";

const TREES = [
  {
    "id": 1,
    "lat": 40.181389,
    "lon": 44.514444,
    "name": "An old birch",
    "height": 18.5,
    "circumference": 1.2,
    "diameter": 7,
    "state": "healthy",
  },
  {
    "id": 128210594892484600,
    "lat": 40.22174748453427,
    "lon": 44.55499326643146,
    "name": "qwe aasd",
    "height": 21.5,
    "circumference": 1.2,
    "diameter": 5,
    "state": "healthy",
  },
  {
    "id": 128242881436717060,
    "lat": 40.16234575628614,
    "lon": 44.62291898117552,
    "name": "test tree",
    "height": 21.5,
    "circumference": 1.2,
    "diameter": 5,
    "state": "healthy",
  },
  {
    "id": 128245117764112380,
    "lat": 40.18018485291529,
    "lon": 44.55350590139917,
    "name": "another tree",
    "height": 21.5,
    "circumference": 1.2,
    "diameter": 5,
    "state": "healthy",
  },
  {
    "id": 128245920759418880,
    "lat": 40.20026462081529,
    "lon": 44.5688510139514,
    "name": "Some tree",
    "height": 21.5,
    "circumference": 1.2,
    "diameter": 5,
    "state": "healthy",
  },
  {
    "id": 128246278395138050,
    "lat": 40.20060879357814,
    "lon": 44.57038489822188,
    "name": "a tree",
    "height": 21.5,
    "circumference": 1.2,
    "diameter": 5,
    "state": "healthy",
  },
  {
    "id": 128246614136590340,
    "lat": 40.20077678203027,
    "lon": 44.56939316739498,
    "name": "ijuhuihg",
    "height": 21.5,
    "circumference": 1.2,
    "diameter": 5,
    "state": "healthy",
  },
  {
    "id": 128258603206447100,
    "lat": 40.21401378262961,
    "lon": 44.51072101214662,
    "name": "test tree",
    "height": 21.5,
    "circumference": 1.2,
    "diameter": 5,
    "state": "healthy",
  },
  {
    "id": 134025978690277380,
    "lat": 40.18484395454571,
    "lon": 44.53205108642578,
    "name": "test 123",
    "height": 21.5,
    "circumference": 1.2,
    "diameter": 5,
    "state": "healthy",
  },
  {
    "id": 134371833565810690,
    "lat": 40.17440928235685,
    "lon": 44.537780817706825,
    "name": "test",
    "height": 21.5,
    "circumference": 1.2,
    "diameter": 5,
    "state": "healthy",
  },
  {
    "id": 134372280418570240,
    "lat": 40.16739627047165,
    "lon": 44.55907357476179,
    "name": "test",
    "height": 21.5,
    "circumference": 1.2,
    "diameter": 5,
    "state": "healthy",
  },
  {
    "id": 134792003752955900,
    "lat": 40.1687178633028,
    "lon": 44.50999259948731,
    "name": "test",
    "height": 21.5,
    "circumference": 1.2,
    "diameter": 5,
    "state": "healthy",
  },
  {
    "id": 134792907214426110,
    "lat": 40.17164148250818,
    "lon": 44.53134298324585,
    "name": "test",
    "height": 21.5,
    "circumference": 1.2,
    "diameter": 5,
    "state": "healthy",
  },
  {
    "id": 134793003121381380,
    "lat": 40.15545808595177,
    "lon": 44.577190755610864,
    "name": "test",
    "height": 21.5,
    "circumference": 1.2,
    "diameter": 5,
    "state": "healthy",
  }
] as ITreeInfo[];

const DETAILS = {
  "id": "134793003121381380",
  "lat": 40.181389,
  "lon": 44.514444,
  "name": "Sycamore",
  "height": 18.5,
  "circumference": 1.2,
  "diameter": 7,
} as TreeDetails;

const preview: Preview = {
  parameters: {
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },

    // Configure viewports.
    // https://storybook.js.org/docs/essentials/viewport
    viewport: {
      viewports: {
        desktop: {
          name: "Desktop",
          styles: {
            width: "1600px",
            height: "900px",
          },
        },
        responsive: {
          name: "Responsive",
          styles: {
            width: "100%",
            height: "100%",
          },
        },
        mobile1: {
          name: "iPhone 5",
          styles: {
            width: "320px",
            height: "568px",
          },
        },
        mobile2: {
          name: "iPhone 14",
          styles: {
            width: "390px",
            height: "844px",
          },
        },
      },
    },

    mockAddonConfigs: {
      globalMockData: [
        {
          url: "/v1/trees?n=n&e=e&s=s&w=w",
          method: "GET",
          status: 200,
          response: {
            trees: TREES,
          },
          delay: 1000,
        },
        {
          url: "/v1/trees",
          method: "POST",
          status: 200,
          response: {
            id: 12345,
            lat: 40.181389,
            lon: 44.514444,
            name: "New Tree",
          } as ITreeInfo,
          delay: 2000,
        },
        {
          url: "/v1/trees/134793003121381380",
          method: "GET",
          status: 200,
          response: DETAILS,
          delay: 500,
        },
        {
          url: "/v1/trees/134793003121381380",
          method: "PUT",
          status: 202,
          delay: 500,
        },
      ],
    },
  },

  decorators: [
    (Story) => (
      <StoryWrapper>
        <MemoryRouter>
          <Story />
        </MemoryRouter>
      </StoryWrapper>
    ),
  ],
};

export default preview;
