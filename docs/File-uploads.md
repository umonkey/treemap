# File Uploads

This file describes how file uploads are working, step by step.


## File storage

Files are stored in a single S3 bucket, where key is the numeric file id (snowflake).

All uploaded files, as well as derived files (like resized thumbnails) are recorded in table `uploads`.
Tree images are also recorded in table `trees_images` -- one record contains the source image id, small and large resized images.


## Upload Workflow

(1) The user uploads file bodies by sending them using `POST /v1/upload` endpoint.
The file is saved in local file system and recorded in the [uploads table][1].
The files are not connected to any tree at this point.
This allows us to upload the files in the background, while the user still fills in new tree details.

(2) When a tree is added, or updated, an array of upload ids is provided to the related API, e.g. `/v1/trees`.
The ids are not saved anywhere directly, but sent to the queue for processing.

(3) The queue consumer reads source files by id, creates all required file versions (e.g., small and large), and saves them in the additional file storage.
The original file record is then deleted from the `uploads` table, but a record with the same id is created in the `files` table.
This is how the source file is moved from uploads to photos.


[1]: https://github.com/umonkey/treemap/wiki/Database-structure#uploads
