/*
 * Copyright (C) 2020 Voldracarno Draconor <ThaFireDragonOfDeath@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::config::ProjectConfig;
use tokio::process::Command;
use std::path::Path;
use std::process::Output;
use serde_json::Value;
use crate::file_api::FileProcessErrorType::{FormatError, UnknownError, PrvGenError, CopyError};
use log::{trace, debug, info, warn, error};

#[derive(Serialize, Deserialize)]
struct FFprobeFormat {
    format_name: String,
    duration: Option<f32>,
    size: u32,
}

#[derive(Serialize, Deserialize)]
struct FFprobeOutput {
    streams: Vec<FFprobeStream>,
    format: FFprobeFormat,
}

#[derive(Serialize, Deserialize)]
struct FFprobeStream {
    codec_name: String,
    width: u32,
    height: u32,
}

pub struct FileProcessError {
    pub error_code: FileProcessErrorType,
    pub error_msg: String,
}

impl FileProcessError {
    pub fn new(error_code: FileProcessErrorType, error_msg: &str) -> FileProcessError {
        FileProcessError {
            error_code,
            error_msg: error_msg.to_owned(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FileProcessErrorType {
    UnknownError,
    FormatError,
    PrvGenError,
    CopyError,
}

async fn generate_preview(ffmpeg_filepath: &str, ffprobe_data: &FFprobeOutput, filename: &str) -> bool {
    let is_image_file = is_image_file(filename);
    let is_video_file = is_video_file(filename);

    if is_image_file || is_video_file {
        let upload_filepath = get_upload_filedrop_path(filename);
        let output_filepath = get_tmp_prv_files_path(filename);
        let width = ffprobe_data.streams.get(0).unwrap().width;
        let height = ffprobe_data.streams.get(0).unwrap().height;
        let crop_resolution = width.min(height);
        let ffmpeg_filter = format!("crop={}:{},scale=150:150", crop_resolution, crop_resolution);
        let video_duration = ffprobe_data.format.duration.unwrap_or(0.0);

        let mut ffmpeg_args : Vec<&str> = Vec::new();
        ffmpeg_args.push("-loglevel");
        ffmpeg_args.push("quiet");


        // If the file is a video and it is long enough, crate the thumbnail from the frame after the first second
        // Else: The thumbnail will be created from the first frame
        if video_duration > 1.1 {
            ffmpeg_args.push("-ss");
            ffmpeg_args.push("1");
        }

        ffmpeg_args.push("-i");
        ffmpeg_args.push(upload_filepath.as_str());
        ffmpeg_args.push("-frames:v");
        ffmpeg_args.push("1");
        ffmpeg_args.push("-filter:v");
        ffmpeg_args.push(ffmpeg_filter.as_str());
        ffmpeg_args.push("-qscale:v");
        ffmpeg_args.push("5");
        ffmpeg_args.push(output_filepath.as_str());

        // Let the OS take care of finding the ffmpeg binary if there is no path provided
        let command = if !ffmpeg_filepath.is_empty() {
            ffmpeg_filepath
        }
        else {
            "ffmpeg"
        };

        let ffmpeg_result : std::io::Result<Output> = Command::new(command)
            .args(ffmpeg_args)
            .output()
            .await;

        if ffmpeg_result.is_ok() && ffmpeg_result.unwrap().status.success() {
            return true;
        }
    }

    return false;
}

pub fn get_url_from_filename(filename: &str) -> String {
    format!("/uploads/{}", filename)
}

pub fn get_preview_url_from_filename(filename: &str) -> String {
    let filename_point_pos = filename.rfind('.').unwrap();
    let (file_name, _file_ext) = filename.split_at(filename_point_pos);

    format!("/prv/{}.{}", file_name, ".jpg")
}

pub fn get_tmp_prv_files_path(filename: &str) -> String {
    format!("./tmp/p0nygramm/preview_files/{}", filename)
}

pub fn get_upload_filedrop_path(filename: &str) -> String {
    format!("./tmp/p0nygramm/upload_files/{}", filename)
}

pub fn is_image_file(filename: &str) -> bool {
    let path_obj = Path::new(filename);
    let file_extension = path_obj.extension();

    if file_extension.is_some() {
        let file_extension = file_extension.unwrap();

        if file_extension == "gif" || file_extension == "jpg" || file_extension == "png" {
            return true;
        }
    }

    return false;
}

pub fn is_video_file(filename: &str) -> bool {
    let path_obj = Path::new(filename);
    let file_extension = path_obj.extension();

    if file_extension.is_some() {
        let file_extension = file_extension.unwrap();

        if file_extension == "mp4" {
            return true;
        }
    }

    return false;
}

// Returns Some(FFprobeOutput) if the file format and codex is valid (if not -> None)
pub async fn probe_file(ffprobe_filepath: &str, upload_filename: &str) -> Option<FFprobeOutput> {
    let upload_filepath = get_upload_filedrop_path(upload_filename);

    let is_image_file = is_image_file(upload_filename);
    let is_video_file = is_video_file(upload_filename);

    // Let the OS take care of finding the ffprobe binary if there is no path provided
    let command = if !ffprobe_filepath.is_empty() {
        ffprobe_filepath
    }
    else {
        "ffprobe"
    };

    if is_image_file || is_video_file {
        let ffprobe_result : tokio::io::Result<Output> = Command::new(command)
            .arg("-loglevel")
            .arg("quiet")
            .arg("-hide_banner")
            .arg("-show_format")
            .arg("-show_streams")
            .arg("-print_format")
            .arg("json")
            .arg(upload_filepath)
            .output()
            .await;

        if ffprobe_result.is_ok() {
            let ffprobe_result : Output = ffprobe_result.unwrap();
            let ffprobe_return_code = ffprobe_result.status;

            // ffprobe terminated with returncode 0
            if ffprobe_return_code.success() {
                let ffprobe_stdout = String::from_utf8(ffprobe_result.stdout);

                if ffprobe_stdout.is_ok() {
                    let ffprobe_stdout = ffprobe_stdout.unwrap();
                    let ffprobe_stdout_json : serde_json::Result<FFprobeOutput> = serde_json::from_str(ffprobe_stdout.as_str());

                    if ffprobe_stdout_json.is_ok() {
                        let ffprobe_stdout_json = ffprobe_stdout_json.unwrap();

                        if is_image_file && probe_image_file(&ffprobe_stdout_json) {
                            return Some(ffprobe_stdout_json);
                        }
                        else if is_video_file && probe_video_file(&ffprobe_stdout_json) {
                            return  Some(ffprobe_stdout_json);
                        }
                    }
                }
            }
        }
    }

    return None;
}

fn probe_image_file(ffprobe_stdout_json: &FFprobeOutput) -> bool {
    let stream_count = ffprobe_stdout_json.streams.len();

    if stream_count == 1 {
        let image_format = ffprobe_stdout_json.format.format_name.as_str();

        // Allow gif, jpeg (image2) and png as container format
        if image_format == "gif" || image_format == "image2" || image_format == "png_pipe" {
            let image_codec = ffprobe_stdout_json.streams.get(0).unwrap().codec_name.as_str();

            if image_codec == "gif" ||  image_codec == "mjpeg" || image_codec == "png" {
                return true;
            }
        }
    }

    return false;
}

fn probe_video_file(ffprobe_stdout_json: &FFprobeOutput) -> bool {
    let stream_count = ffprobe_stdout_json.streams.len();

    if stream_count == 1 || stream_count == 2 {
        let video_format = ffprobe_stdout_json.format.format_name.as_str();

        // Allow mp4 container format
        if video_format == "mov,mp4,m4a,3gp,3g2,mj2" {
            let video_codec = ffprobe_stdout_json.streams.get(0).unwrap().codec_name.as_str();

            // Allow h264 video codec
            if video_codec == "h264" {
                // if the video has an audio stream: Check if it uses the aac codec
                if stream_count == 2 {
                    let audio_codec = ffprobe_stdout_json.streams.get(1).unwrap().codec_name.as_str();

                    if audio_codec == "aac" {
                        return true;
                    }
                }
                else {
                    return true;
                }
            }
        }
    }

    return false;
}

pub async fn process_file(config: &ProjectConfig, filename: &str) -> Result<(), FileProcessError> {
    let ffmpeg_filepath = config.filesystem_config.ffmpeg_path.get_value();
    let ffprobe_path = config.filesystem_config.ffprobe_path.get_value();
    let format_data = probe_file(ffprobe_path.as_str(), filename).await;
    let tmp_upload_filepath = get_upload_filedrop_path(filename);
    let tmp_upload_prv_filepath = get_tmp_prv_files_path(filename);

    let mut return_val : Result<(), FileProcessError> = Err(FileProcessError::new(UnknownError, "Unbekannter Fehler"));

    if format_data.is_some() {
        let format_data = format_data.unwrap();
        let generate_preview_success = generate_preview(ffmpeg_filepath.as_str(), &format_data, filename).await;

        if generate_preview_success {
            let srv_upload_directory = config.filesystem_config.uploads_path.get_value();
            let srv_upload_prv_directory = config.filesystem_config.uploads_prv_path.get_value();
            let srv_upload_filepath = format!("{}/{}", srv_upload_directory, filename);
            let srv_upload_prv_filepath = format!("{}/{}", srv_upload_prv_directory, filename);

            let cpy_upload_success : core::result::Result<u64, _> = tokio::fs::copy(tmp_upload_filepath.as_str(), srv_upload_filepath.as_str()).await;
            let cpy_upload_prv_success : core::result::Result<u64, _> = tokio::fs::copy(tmp_upload_prv_filepath.as_str(), srv_upload_prv_filepath.as_str()).await;

            if cpy_upload_success.is_ok() && cpy_upload_prv_success.is_ok() {
                return_val = Ok(());
            }
            else {
                return_val = Err(FileProcessError::new(CopyError, "Fehler beim Kopieren der Dateien ins Serververzeichnis"));
            }
        }
        else {
            return_val = Err(FileProcessError::new(PrvGenError, "Fehler beim Erzeugen der Vorschaubilder"));
        }
    }
    else {
        return_val = Err(FileProcessError::new(FormatError, "Format der Datei wird nicht akzepziert"));
    }

    let rm_tmp_upload_success : tokio::io::Result<()> = tokio::fs::remove_file(tmp_upload_filepath.as_str()).await;
    let rm_tmp_upload_prv_success : tokio::io::Result<()> = tokio::fs::remove_file(tmp_upload_prv_filepath.as_str()).await;

    if rm_tmp_upload_success.is_err() {
        warn!("Temoräre Datei konnte nicht gelöscht werden: {}", tmp_upload_filepath.as_str());
    }

    if !rm_tmp_upload_prv_success.is_err() {
        warn!("Temoräre Datei konnte nicht gelöscht werden: {}", tmp_upload_prv_filepath.as_str());
    }

    return return_val;
}