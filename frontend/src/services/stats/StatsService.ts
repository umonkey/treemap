import { mainBus } from "@/bus";
import { useStore } from "@/store";
import { treeMapService } from "@/services/api";

class StatsService {
  public constructor() {
  }

  public start() {
    console.debug("[stats] Stats service initialized.");
    mainBus.on("initialize", () => this.reload());
    mainBus.on("reload_map", () => this.reload());
  }

  public stop() {
    console.debug("[stats] Stats service uninitialized.");
    mainBus.off("initialize", () => this.reload());
    mainBus.off("reload_map", () => this.reload());
  }

  private async reload() {
    try {
      const res = await treeMapService.getTreeStats();
      useStore.getState().setStats(res);
      console.debug("[stats] Stats reloaded.");
    } catch (e) {
      console.error("[stats] Error fetching stats: ", e);
    }
  }
}

export const statsService = new StatsService();
