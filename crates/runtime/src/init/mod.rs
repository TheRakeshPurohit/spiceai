/*
Copyright 2024 The Spice.ai OSS Authors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

     https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

//! Code needed to initialize the runtime

pub(crate) mod catalog;
pub(crate) mod dataset;
pub(crate) mod embedding;
pub(crate) mod extension;
pub(crate) mod llm;
pub(crate) mod metrics;
pub(crate) mod model;
pub(crate) mod pods_watcher;
pub(crate) mod results_cache;
pub(crate) mod task_history;
pub(crate) mod tool;
pub(crate) mod view;
