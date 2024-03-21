import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';

import { HomePage } from "./HomePage";
import { ITreeInfo } from "../../../types";

const meta = {
  title: 'Pages/HomePage',
  component: HomePage,
  parameters: {
    layout: 'fullscreen', // centered
    mockData: [
      {
        url: "http://localhost:8000/v1/trees?n=n&e=e&s=s&w=w",
        method: "GET",
        status: 200,
        response: {
          trees: [
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
          ] as ITreeInfo[],
        },
      },
    ],
  },
  args: {
    onClick: fn(),
  },
} satisfies Meta<typeof HomePage>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: { },
};

export const Phone: Story = {
  args: { },
};