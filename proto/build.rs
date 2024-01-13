// Copyright 2023 The Trusted Computations Platform Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::Result;

fn main() -> Result<()> {
    micro_rpc_build::compile(
        &[
            "src/endpoint.proto",
            "src/atomic_counter.proto",
            "src/ledger.proto",
        ],
        &["src"],
        micro_rpc_build::CompileOptions {
            bytes: vec![
                ".runtime.endpoint.StartReplicaRequest".to_string(),
                ".runtime.endpoint.DeliverMessage".to_string(),
                ".runtime.endpoint.DeliverSnapshotRequest".to_string(),
                ".runtime.endpoint.DeliverSnapshotResponse".to_string(),
                ".runtime.endpoint.ExecuteProposalRequest".to_string(),
                ".runtime.endpoint.ExecuteProposalResponse".to_string(),
                ".runtime.endpoint.Entry".to_string(),
                ".examples.atomic_counter.CounterSnapshotValue".to_string(),
                ".examples.atomic_counter.CounterRequest".to_string(),
            ],
            ..Default::default()
        },
    );
    Ok(())
}
