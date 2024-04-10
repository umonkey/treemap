import type { Meta, StoryObj } from '@storybook/react';

import { SideBar } from "./SideBar";

import "./stories.css";

const meta = {
  title: 'Layout/SideBar',
  component: SideBar,
  parameters: {
    layout: 'fullscreen',
  },
  render: () => (
    <div className="SamplePage">
      <div className="SampleBody">Main content goes here.</div>
      <SideBar>
        <h2>SideBar</h2>
        <p>SideBar content goes here. Not sure yet how big can it get. Probably should keep it as tight as possible.</p>
        <p>On mobile, this should be a map overlay.</p>
      </SideBar>
    </div>
  ),
} satisfies Meta<typeof SideBar>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
