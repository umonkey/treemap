import type { Meta, StoryObj } from '@storybook/react';

import { HomePage } from "./HomePage";
import { ITreeInfo } from "@/types";

const TREES = [
  {
    id: 1,
    lat: 40.181389,
    lon: 44.514444,
  },
  {
    id: 2,
    lat: 40.182389,
    lon: 44.515444,
  },
] as ITreeInfo[];

const meta = {
  title: 'Pages/HomePage',
  component: HomePage,
  parameters: {
    layout: 'fullscreen', // centered
    mockData: [
      {
        url: "/v1/trees?n=n&e=e&s=s&w=w",
        method: "GET",
        status: 200,
        response: {
          trees: TREES,
        },
      },
    ],
  },
  args: { },
} satisfies Meta<typeof HomePage>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: { },
  parameters: {
    viewport: {
      defaultViewport: "desktop",
    },
  },
};

export const Phone: Story = {
  args: { },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
