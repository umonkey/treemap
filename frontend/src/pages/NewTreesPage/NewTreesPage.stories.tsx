import type { Meta, StoryObj } from '@storybook/react';

import { SAMPLE_TREES } from "@/sample-data";

import { NewTreesPage } from "./NewTreesPage";

const meta = {
  title: 'Pages/NewTreesPage',
  component: NewTreesPage,
  parameters: {
    layout: "fullscreen",
    mockData: [
      {
        url: "/v1/trees/new?count=count&skip=skip",
        method: "GET",
        status: 200,
        delay: 500,
        response: {
          trees: SAMPLE_TREES,
        },
      },
    ],
  },
  args: { },
} satisfies Meta<typeof NewTreesPage>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    count: 50,
    skip: 0,
  },
};
