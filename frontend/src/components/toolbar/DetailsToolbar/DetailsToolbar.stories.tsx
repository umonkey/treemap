import type { Meta, StoryObj } from '@storybook/react';

import { DetailsToolbar } from "./DetailsToolbar";

const meta = {
  title: 'Toolbar/DetailsToolbar',
  component: DetailsToolbar,
  parameters: {
    layout: "fullscreen",
    viewport: {
      defaultViewport: "mobile1",
    },
  },
} satisfies Meta<typeof DetailsToolbar>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    id: "134793003121381380",
  },
};
