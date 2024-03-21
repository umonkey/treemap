import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';

import { Map } from "./Map";
import { ITreeInfo } from "../../../types";

const meta = {
  title: 'Maps/Map',
  component: Map,
  parameters: {
    layout: 'fullscreen', // centered
    mockData: [
      {
        url: "/v1/trees?n=n&e=e&s=s&w=w",
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
    onAddTree: fn(),
  },
} satisfies Meta<typeof Map>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: { },
};

export const Phone: Story = {
  args: { },
};
