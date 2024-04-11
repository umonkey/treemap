import type { Meta, StoryObj } from '@storybook/react';

import { DetailsPage } from "./DetailsPage";

const TREE_ID = 134793003121381380;

const meta = {
  title: 'Pages/DetailsPage',
  component: DetailsPage,
  parameters: {
    layout: 'padded', // centered
  },
  args: { },
} satisfies Meta<typeof DetailsPage>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    id: TREE_ID,
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    id: TREE_ID,
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
