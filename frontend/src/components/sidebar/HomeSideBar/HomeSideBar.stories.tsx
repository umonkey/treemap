// Global imports.
import type { Meta, StoryObj } from '@storybook/react';

// Project imports.
import { WithHeader, WithSidebar, SideBar } from "@/components";

// Local imports.
import { HomeSideBar } from "./HomeSideBar";

const meta = {
  title: 'Sidebar/HomeSideBar',
  component: HomeSideBar,
  parameters: {
    layout: 'fullscreen',
  },
  render: () => (
    <WithHeader>
      <WithSidebar>
        <div style={{ width: "100%", padding: "20px" }}>Main contents would be here.</div>
        <SideBar>
          <HomeSideBar />
        </SideBar>
      </WithSidebar>
    </WithHeader>
  ),
} satisfies Meta<typeof HomeSideBar>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: { },
};
