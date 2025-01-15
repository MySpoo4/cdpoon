use cdpoon::*;

#[tokio::main]
async fn main() {
    let client = client::CdpClient::custom("localhost", 9222);

    // Connect to the first tab
    client.connect_to_tab(0).await.unwrap();

    let root_node_response = client
        .send(models::Cmd {
            method: "Page.enable",
            params: params!(),
        })
        .await
        .unwrap();
    println!("{:#?}", root_node_response);
    // Navigate to the desired URL
    let a = client
        .send(models::Cmd {
            method: "Page.navigate",
            params: params!("url" => "https://www.novelupdates.com"),
        })
        .await
        .unwrap();
    println!("{:#?}", a);

    let frame_id = a.get("frameId").unwrap().as_str().unwrap();
    println!("{:#?}", frame_id);
    let b = client
        .wait_for_event(models::Event {
            method: "Page.frameStoppedLoading",
            params: params!("frameId" => frame_id),
        })
        .await
        .unwrap();
    println!("b: {:#?}", b);
    let root_node_response = client
        .send(models::Cmd {
            method: "DOM.getDocument",
            params: params!(
                // "pierce" => true,
                "depth" => 1
            ),
        })
        .await
        .unwrap();

    // println!("{:?}", root_node_response);
    //
    // let res = client
    //     .send(models::Cmd {
    //         method: "Runtime.enable",
    //         params: params!(),
    //     })
    //     .await
    //     .unwrap();
    //
    // let xpath_expr = r#"
    //     document.evaluate("//span[text()="Novel Updates"]",document,null,XPathResult.ORDERED_NODE_SNAPSHOT_TYPE,null).snapshotItem(0)
    // "#;
    let res = client
        .send(models::Cmd {
            method: "DOM.enable",
            params: params!(),
        })
        .await
        .unwrap();
    println!("{:?}", res);

    let res = client
        .send(models::Cmd {
            method: "DOM.performSearch",
            params: params!(
                "query" => "//*[contains(text(), 'ElloTL')]",
                "includeUserAgentShadowDOM" => true
            ),
        })
        .await
        .unwrap();

    println!("res: {:?}", res);

    let search_id = res["searchId"].as_str().unwrap();
    println!("search_id: {}", search_id);
    let result_count = res["resultCount"].as_i64().unwrap_or(0);
    let res = client
        .send(models::Cmd {
            method: "DOM.getSearchResults",
            params: params!(
                "searchId" => search_id,
                "fromIndex" => 0,
                "toIndex" => result_count
            ),
        })
        .await
        .unwrap();

    if let Some(node_ids) = res["nodeIds"].as_array() {
        for node_id in node_ids {
            println!("Node ID: {:?}", node_id);
            // Get the outer HTML of the element
            let html_response = client
                .send(models::Cmd {
                    method: "DOM.getOuterHTML",
                    params: params!("nodeId" => node_id.as_i64()),
                })
                .await
                .unwrap();
            println!("{:#?}", html_response);
        }
    } else {
        println!("No node IDs found.");
    }

    // let res = client
    //     .send(models::Cmd {
    //         method: "Runtime.evaluate",
    //         params: params!(
    //             "expression" => r#"
    //                 const snapshot = document.evaluate(
    //                     "//span[text()='Novel Updates']",
    //                     document,
    //                     null,
    //                     XPathResult.ORDERED_NODE_SNAPSHOT_TYPE,
    //                     null
    //                 )
    //                 const results = []
    //                 for (let i = 0; i < snapshot.snapshotLength; i++) {
    //                     results.push(snapshot.snapshotItem(i).outerHTML)
    //                 }
    //                 results
    //             "#,
    //             "returnByValue" => true
    //         ),
    //     })
    //     .await
    //     .unwrap();
    // println!("{:?}", res["result"]["result"]["value"]);
    // let res = client
    //     .send(models::Cmd {
    //         method: "Runtime.evaluate",
    //         params: params!(
    //             "expression" => r#"document.evaluate("//span[text()='Novel Updates']", document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null).singleNodeValue.outerHTML"#,
    //             "returnByValue" => true
    //         ),
    //     })
    //     .await
    //     .unwrap();
    // println!("{:?}", res);

    // println!(
    //     "{:?}",
    //     res["result"]["result"]["objectId"].as_str().unwrap()
    // );
    //
    // let object_id = res["result"]["result"]["objectId"].as_str().unwrap();
    // let res = client
    //     .send(models::Cmd {
    //         method: "DOM.describeNode",
    //         params: params!(
    //             "objectId" => object_id,
    //             "depth" => -1
    //         ),
    //     })
    //     .await
    //     .unwrap();
    //
    // println!("{:?}", res);
    // println!("{:?}", res["result"]["node"]["nodeId"]);
    // // let id = res["result"]["node"]["nodeId"].as_i64().unwrap();
    // // let html_response = client
    // //     .send(models::Cmd {
    // //         method: "DOM.getOuterHTML",
    // //         params: params!("nodeId" => id),
    // //     })
    // //     .await
    // //     .unwrap();
    // //
    // // println!("{:?}", html_response);
    //
    // // let res = client
    // //     .send(models::Cmd {
    // //         method: "Runtime.callFunctionOn",
    // //         params: params!(
    // //             "objectId" => object_id,
    // //             "functionDeclaration" => "function() { return this.outerHTML; }"
    // //         ),
    // //     })
    // //     .await
    // //     .unwrap();
    // //
    // // println!("{:?}", res);
    //
    // // // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    // // // // Get the root document node
    // // //
    // // // // Get the root document node
    // // let root_node_response = client
    // //     .send(models::Cmd {
    // //         method: "DOM.getDocument",
    // //         params: params!(
    // //             // "pierce" => true,
    // //             // "depth" => -1
    // //         ),
    // //     })
    // //     .await
    // //     .unwrap();
    // // //
    // // // println!("{:?}", root_node_response);
    // // let root_node_id = root_node_response["result"]["root"]["nodeId"]
    // //     .as_i64()
    // //     .unwrap();
    // // println!("Root Node ID: {}", root_node_id);
    // // // //
    // // // // // Query the desired element (e.g., #primary-content)
    // // let query_response = client
    // //     .send(models::Cmd {
    // //         method: "DOM.querySelector",
    // //         params: params!(
    // //             "nodeId" => root_node_id,
    // //             "selector" => "a"
    // //         ),
    // //     })
    // //     .await
    // //     .unwrap();
    // //
    // // println!("{:?}", query_response);
    // // let element_node_id = query_response["result"]["nodeId"].as_i64().unwrap();
    // // println!("Element Node ID: {}", element_node_id);
    // //
    // // // Get the outer HTML of the element
    // // let html_response = client
    // //     .send(models::Cmd {
    // //         method: "DOM.getOuterHTML",
    // //         params: params!("nodeId" => element_node_id),
    // //     })
    // //     .await
    // //     .unwrap();
    // //
    // // println!("{:?}", html_response);
    // // // // // let outer_html = html_response["result"]["outerHTML"].as_str().unwrap();
    // // // // // println!("Outer HTML of #primary-content:\n{}", outer_html);
}
