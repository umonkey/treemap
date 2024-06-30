import type { Meta, StoryObj } from '@storybook/react';

import { SAMPLE_TREES } from "@/sample-data";

import { AddedTreesPage } from "./AddedTreesPage";

const meta = {
  title: 'Pages/AddedTreesPage',
  component: AddedTreesPage,
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
} satisfies Meta<typeof AddedTreesPage>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    count: 50,
    skip: 0,
  },
};
