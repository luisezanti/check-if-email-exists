// Reacher - Email Verification
// Copyright (C) 2018-2023 Reacher

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Main entry point of the `reacher_backend` binary. It has two `main`
//! functions, depending on whether the `bulk` feature is enabled or not.

use check_if_email_exists::LOG_TARGET;
#[cfg(feature = "worker")]
use futures::try_join;
use tracing::info;

#[cfg(feature = "worker")]
use reacher_backend::worker::run_worker;
use reacher_backend::{
	http::run_warp_server,
	sentry_util::{setup_sentry, CARGO_PKG_VERSION},
};

/// Run a HTTP server using warp with bulk endpoints.
/// If the worker feature is enabled, this function will also start a worker
/// that listens to an AMQP message queue.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	// Initialize logging.
	tracing_subscriber::fmt::init();
	info!(target: LOG_TARGET, version=?CARGO_PKG_VERSION, "Running Reacher");

	// Setup sentry bug tracking.
	let _guard: sentry::ClientInitGuard = setup_sentry();

	let _http_server = run_warp_server();

	#[cfg(feature = "worker")]
	try_join!(_http_server, run_worker())?;

	Ok(())
}
