use std::str::FromStr;
use std::sync::Arc;

use opcua::client::prelude::*;
use opcua::sync::RwLock;

fn main() {
    // Make the client configuration
    let mut client = ClientBuilder::new()
        .application_name("Simple Client")
        .application_uri("urn:SimpleClient")
        .product_uri("urn:SimpleClient")
        .trust_server_certs(true)
        .create_sample_keypair(true)
        .session_retry_limit(3)
        .client()
        .unwrap();

    if let Ok(session) = client.connect_to_endpoint(
        (
            "opc.tcp://192.168.0.133:53530/OPCUA/SimulationServer",
            SecurityPolicy::None.to_str(),
            MessageSecurityMode::None,
            UserTokenPolicy::anonymous(),
        ),
        IdentityToken::Anonymous,
    ) {
        println!("server connected");
        if let Err(result) = subscribe_to_variables(session.clone(), 10) {
            println!(
                "ERROR: Got an error while subscribing to variables - {}",
                result
            );
        } else {
            // Loops forever. The publish thread will call the callback with changes on the variables
            let _ = Session::run(session);
        }
    }
}

fn subscribe_to_variables(session: Arc<RwLock<Session>>, ns: u16) -> Result<(), StatusCode> {
    let session = session.read();
    let nodes_to_read = vec![ReadValueId {
        node_id: NodeId { namespace: 10, identifier: Identifier::from_str("i=1004").unwrap() },
        attribute_id: 1,
        index_range: UAString::null(),
        data_encoding: QualifiedName::null(),

    }];
    let timestamps_to_return = TimestampsToReturn::Source;
    let max_age = i32::MAX as f64;
    let data_value = session.read(nodes_to_read.as_slice(), timestamps_to_return, max_age)?;
    dbg!(&data_value);
    // Creates a subscription with a data change callback
    let subscription_id = session.create_subscription(
        2000.0,
        10,
        30,
        0,
        0,
        true,
        DataChangeCallback::new(|changed_monitored_items| {
            println!("Data change from server:");
            changed_monitored_items
                .iter()
                .for_each(|item| print_value(item));
        }),
    )?;
    println!("Created a subscription with id = {}", subscription_id);

    // Create some monitored items
    let items_to_create: Vec<MonitoredItemCreateRequest> = ["i=1004", "i=1005", "i=1006"]
        .iter()
        .map(|v| NodeId::new(ns, Identifier::from_str(v).unwrap()).into())
        .collect();
    let _ = session.create_monitored_items(
        subscription_id,
        TimestampsToReturn::Both,
        &items_to_create,
    )?;

    Ok(())
}

fn print_value(item: &MonitoredItem) {
    let node_id = &item.item_to_monitor().node_id;
    let data_value = item.last_value();
    if let Some(ref value) = data_value.value {
        println!("Item \"{}\", Value = {:?}", node_id, value);
    } else {
        println!(
            "Item \"{}\", Value not found, error: {}",
            node_id,
            data_value.status.as_ref().unwrap()
        );
    }
}
