import type { Meta, StoryObj } from '@storybook/react';

import { SAMPLE_TREES } from "@/sample-data";

import { AddTreeDialog } from "./AddTreeDialog";

const meta = {
  title: 'Dialogs/AddTreeDialog',
  component: AddTreeDialog,
  parameters: {
    layout: 'padded', // centered, fullscreen
    mockData: [
      {
        url: "/v1/trees",
        method: "POST",
        status: 200,
        response: SAMPLE_TREES[0],
      },
    ],
  },
} satisfies Meta<typeof AddTreeDialog>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    points: [
      {
        lat: 40.181389,
        lon: 44.514444,
      },
    ],
  },
};
