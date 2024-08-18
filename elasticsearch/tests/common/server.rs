/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
// From reqwest crate
// Licensed under Apache License, Version 2.0
// https://github.com/seanmonstar/reqwest/blob/master/LICENSE-APACHE

use std::{
    future::Future, sync::mpsc as std_mpsc, thread, time::Duration,
};
use std::future::IntoFuture;
use std::net::SocketAddr;
use tokio::sync::oneshot;

use tokio::runtime;

pub struct Server {
    addr: SocketAddr,
    panic_rx: std_mpsc::Receiver<()>,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

impl Server {
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }

        if !::std::thread::panicking() {
            self.panic_rx
                .recv_timeout(Duration::from_secs(3))
                .expect("test server should not panic");
        }
    }
}

pub fn http<F, Fut>(func: F) -> Server
where
    F: Fn(axum::extract::Request) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = axum::response::Response> + Send + 'static,
{
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    //Spawn new runtime in thread to prevent reactor execution context conflict
    thread::spawn(move || {
        let rt = runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let (srv, addr) = rt.block_on(async {
            let bind_addr: SocketAddr = ([127, 0, 0, 1], 0).into();
            let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();
            let local_addr = listener.local_addr().unwrap();
            let app = axum::Router::new()
                .fallback(func); // handle everything

            let server = axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    let _ = shutdown_rx.await;
                });

            (server.into_future(), local_addr)
        });

        let (panic_tx, panic_rx) = std_mpsc::channel();
        let tname = format!(
            "test({})-support-server",
            thread::current().name().unwrap_or("<unknown>")
        );
        thread::Builder::new()
            .name(tname)
            .spawn(move || {
                rt.block_on(srv).unwrap();
                let _ = panic_tx.send(());
            })
            .expect("thread spawn");

        Server {
            addr,
            panic_rx,
            shutdown_tx: Some(shutdown_tx),
        }
    })
    .join()
    .unwrap()
}
