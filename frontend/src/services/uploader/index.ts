/**
 * Background file upload queue.
 *
 * [x] Receives new upload requests via the bus.
 * [x] Sends files to the API while the user can navigate around.
 * [x] Maintains global file upload progress.
 * [ ] Reports when all uploads are done, to show a toast.
 * [ ] Checks file upload status.
 * [ ] When the file is ready, notifies the UI that a tree has new files.
 * [ ] Uses IndexedDB to keep the upload queue between sessions (app crashes etc).
 */

import { IFileUploadRequest } from "@/types";
import { treeMapService } from "@/services";
import { mainBus } from "@/bus";

class FileUploader {
  private queue: IFileUploadRequest[] = [];

  // Make sure we only run one queue processor thread.
  private running: boolean = false;

  private totalSize: number = 0;
  private totalSent: number = 0;
  private currentSent: number = 0;

  public constructor() {
    console.debug("[upload] File uploader initialized.");
    mainBus.on("upload_image", (req: IFileUploadRequest) => this.add(req));
  }

  public finish() {
    console.debug("[upload] Finishing.");
    this.queue = [];
  }

  public add(req: IFileUploadRequest) {
    console.debug(`[upload] File added to queue, tree=${req.tree}, size=${req.file.size}.`);
    this.totalSize += req.file.size;
    this.queue.push(req);
  }

  public async run() {
    if (this.running) {
      console.warn("[upload] Queue handler already running.");
      return;
    }

    this.running = true;
    console.info("[upload] Queue handler started.");

    for (;;) {
      const req = this.queue.shift();

      if (req) {
        try {
          await this.uploadSingleFile(req);
          await this.sleep(1000);

          // Finished all processing?
          if (this.queue.length === 0) {
            this.totalSent = this.totalSize;
            this.reportProgress();

            await this.sleep(1000);

            this.totalSize = 0;
            this.totalSent = 0;
            this.reportProgress();

            mainBus.emit("upload_finished");
          }
        } catch (e) {
          console.error(`[upload] Error uploading file: ${e}, will retry in 5 seconds.`);
          this.queue.push(req);

          await this.sleep(5000);
        }
      } else {
        await this.sleep(1000);
      }
    }
  }

  private async uploadSingleFile(req: IFileUploadRequest) {
    this.currentSent = 0;
    
    await treeMapService.uploadImage({
      tree_id: req.tree,
      file: req.file,
      progress: (sent: number) => {
        this.currentSent += sent;
        this.reportProgress();
        console.debug(`[upload] file=${req.file.name} sent=${sent}`);
      }
    });

    this.totalSent += req.file.size;
    this.currentSent = 0;
    
    this.reportProgress();

    console.info(`[upload] File added to tree ${req.tree} successfully.`);
  }

  private sleep(time: number) {
    return new Promise((resolve) => setTimeout(resolve, time));
  }

  private reportProgress() {
    const percentage = this.getPercentage();
    console.debug(`[upload] Progress=${percentage}%`);
    mainBus.emit("upload_progress", percentage);
  }

  private getPercentage(): number {
    if (this.totalSize === 0) {
      return 0;
    }

    const sent = this.totalSent + this.currentSent;
    return sent * 100 / this.totalSize;
  }
}

export const fileUploader = new FileUploader();
