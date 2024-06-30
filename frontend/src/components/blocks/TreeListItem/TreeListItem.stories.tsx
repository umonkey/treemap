import type { Meta, StoryObj } from '@storybook/react';

import { SAMPLE_TREES } from "@/sample-data";

import { TreeListItem } from "./TreeListItem";

const meta = {
  title: 'Blocks/TreeListItem',
  component: TreeListItem,
  parameters: {
    layout: 'padded', // centered, fullscreen
  },
} satisfies Meta<typeof TreeListItem>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    tree: SAMPLE_TREES[0],
  },
};
