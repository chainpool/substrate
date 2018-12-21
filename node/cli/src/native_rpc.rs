// Copyright 2018 Chainpool

use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use rpc;
use substrate_service::{ComponentExHash, TransactionPool, Components,
     ComponentClient, ComponentBlock, TaskExecutor};
use rpc::apis::system::SystemInfo;
use client::runtime_api::Metadata;

fn maybe_start_server<T, F>(address: Option<SocketAddr>, start: F) -> Result<Option<T>, io::Error>
where
    F: Fn(&SocketAddr) -> Result<T, io::Error>,
{
    Ok(match address {
        Some(mut address) => Some(start(&address).or_else(|e| match e.kind() {
            io::ErrorKind::AddrInUse | io::ErrorKind::PermissionDenied => {
                warn!("Unable to bind server to {}. Trying random port.", address);
                address.set_port(0);
                start(&address)
            }
            _ => Err(e),
        })?),
        None => None,
    })
}

pub fn start_rpc<C: Components> (
    client: Arc<ComponentClient<C>>,
    network: Arc<network::SyncProvider<ComponentBlock<C>>>,
    should_have_peers: bool,
    rpc_system_info: SystemInfo,
    rpc_http: Option<SocketAddr>,
    rpc_ws: Option<SocketAddr>,
    task_executor: TaskExecutor,
    transaction_pool: Arc<TransactionPool<C::TransactionPoolApi>>,
)
 -> (
    Result<Option<rpc::HttpServer>, io::Error>,
    Result<Option<rpc::WsServer>, io::Error>,
)
    where C::RuntimeApi: Metadata<ComponentBlock<C>> {
    let handler = || {
        let client = client.clone();
        let subscriptions = rpc::apis::Subscriptions::new(task_executor.clone());
        let chain = rpc::apis::chain::Chain::new(client.clone(), subscriptions.clone());
        let state = rpc::apis::state::State::new(client.clone(), subscriptions.clone());
        let author =
            rpc::apis::author::Author::new(client.clone(), transaction_pool.clone(), subscriptions);
        let system = rpc::apis::system::System::new(
            rpc_system_info.clone(),
            network.clone(),
            should_have_peers,
        );
        rpc::rpc_handler::<ComponentBlock<C>, ComponentExHash<C>, _, _, _, _>(
            state, chain, author, system,
        )
    };

    let rpc_http: Result<Option<rpc::HttpServer>, io::Error> = maybe_start_server(rpc_http, |address| rpc::start_http(address, handler()));
    let rpc_ws: Result<Option<rpc::WsServer>, io::Error> = maybe_start_server(rpc_ws, |address| rpc::start_ws(address, handler()));
    (rpc_http, rpc_ws)
}
