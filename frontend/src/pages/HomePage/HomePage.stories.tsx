import type { Meta, StoryObj } from '@storybook/react';

import { HomePage } from "./HomePage";

const meta = {
  title: 'Pages/HomePage',
  component: HomePage,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: {
    searchQuery: "Thuja cordata",
  },
} satisfies Meta<typeof HomePage>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: { },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
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
