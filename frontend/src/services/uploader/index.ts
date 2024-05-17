/**
 * Background file upload queue.
 *
 * [x] Receives new upload requests via the bus.
 * [x] Sends files to the API while the user can navigate around.
 * [x] Maintains global file upload progress.
 * [x] Reports when all uploads are done, to show a toast.
 * [x] Checks file upload status.
 * [x] When the file is ready, notifies the UI that a tree has new files.
 * [ ] Uses IndexedDB to keep the upload queue between sessions (app crashes etc).
 *
 * How this works.
 *
 * (0) The application (App class) loads this file, which initializes the FileUploader class
 * by creating the queue and initializing the progress tracker.  It then calls the run() method
 * to start the queue processor.
 *
 * (1) Some UI element, like the photo upload button, uses the useFileUploader() hook
 * to send files to the queue.  That is done by sending the "upload_image" signal
 * to the main event bus.
 *
 * (2) An event handler in the FileUploader class picks up the signal and adds the file to the queue.
 * The queue processing loop, which runs asynchronously, picks that file up and sends it to the API.
 *
 * After the file is uploaded, the API returns a file id.  We then check that files's status for up to
 * 10 seconds, to see if it is ready (like finished resizing).  When it is, we notify the UI by sending
 * an "upload_ready" signal to the main bus.
 */

import { toast } from "react-hot-toast";

import { IFileUploadRequest, IFileUploadResponse } from "@/types";
import { treeMapService } from "@/services";
import { getDebug, setDebug } from "@/utils/storage";
import { mainBus } from "@/bus";

class FileUploader {
  private queue: IFileUploadRequest[] = [];

  // Make sure we only run one queue processor thread.
  private running: boolean = false;

  // Enabled on certain search requests.
  private debug: boolean = false;

  private totalSize: number = 0;
  private totalSent: number = 0;
  private currentSent: number = 0;

  public constructor() {
    console.debug("[upload] File uploader initialized.");
    mainBus.on("upload_image", (req: IFileUploadRequest) => this.add(req));
    mainBus.on("search", (query: string) => this.search(query));

    this.debug = getDebug();
  }

  public finish() {
    console.debug("[upload] Cleaning up.");
    this.queue = [];
  }

  public add(req: IFileUploadRequest) {
    this.debugMessage(`File ${req.file.name} added to queue.`);

    this.totalSize += req.file.size;
    this.queue.push(req);
    this.reportProgress();

    this.run();
  }

  public search(query: string) {
    if (query.toLowerCase().includes("nodebug")) {
      this.debug = false;
      this.debugMessage("Debug mode enabled.");
    } else if (query.toLowerCase().includes("debug")) {
      this.debug = true;
      this.debugMessage("Debug mode enabled.");
    }

    setDebug(this.debug);
  }

  public async run() {
    if (this.running) {
      console.debug("[upload] Queue handler already running.");
      return;
    }

    this.running = true;
    console.info("[upload] Queue handler started.");

    try {
      await this.run_loop();
    } finally {
      this.running = false;
      console.info("[upload] Queue handler stopped.");
    }
  }

  /**
   * Check when the file is ready, ask the details page to reload.
   */
  private async checkFile(file_id: string, tree_id: string) {
    console.debug(`[upload] Checking file ${file_id} status for tree ${tree_id}...`);

    for (let n = 0; n < 30; n++) {
      const status = await treeMapService.getFileStatus(file_id);

      if (status.ready) {
        console.debug(`[upload] File ${file_id} is ready.`);

        mainBus.emit("upload_ready", {
          file_id: file_id,
          tree_id: tree_id,
        });

        return;
      }

      await this.sleep(1000);
    }

    console.debug(`[upload] File ${file_id} did not become ready in time.`);
  }

  private async uploadSingleFile(req: IFileUploadRequest): Promise<IFileUploadResponse> {
    this.currentSent = 0;

    const res = await treeMapService.uploadImage({
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

    this.debugMessage(`File ${req.file.name} uploaded successfully.`);

    return res;
  }

  private async run_loop() {
    for (;;) {
      const req = this.queue.shift();

      if (req) {
        try {
          this.debugMessage(`Uploading ${req.file.name} to tree ${req.tree}.`);

          const res = await this.uploadSingleFile(req);
          this.checkFile(res.id, req.tree);

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
          this.debugMessage(`Error uploading file: ${e}, will retry in 5 seconds.`);

          console.error(`[upload] Error uploading file: ${e}, will retry in 5 seconds.`);

          this.queue.push(req);

          await this.sleep(5000);
        }
      } else {
        await this.sleep(1000);
      }
    }
  }

  private sleep(time: number) {
    return new Promise((resolve) => setTimeout(resolve, time));
  }

  private reportProgress() {
    const percentage = this.getPercentage();
    console.debug(`[upload] Progress=${percentage}%`);
    mainBus.emit("upload_progress", percentage);
  }

  /**
   * Calculate current upload progress.
   *
   * Reports 0% only if there's no pending uploads.
   * Otherwise, reports at least 1% to make sure the progress bar is visible.
   */
  private getPercentage(): number {
    if (this.totalSize === 0) {
      return 0;
    }

    const sent = this.totalSent + this.currentSent;
    return Math.max(1, sent * 100 / this.totalSize);
  }

  private debugMessage(msg: string) {
    console.debug(msg);

    this.debug && toast(msg, {
      icon: '🔍',
    });
  }
}

export const fileUploader = new FileUploader();
