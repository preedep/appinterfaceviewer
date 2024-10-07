use std::collections::HashSet;
use log::info;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use crate::models::appdata::{AppInformation, CommunicationMethod};

// Recursive DFS function to find all routes that match the desired route
// Function to start search from every node if start and goal are None
// Recursive DFS function to find all routes that match the desired route
pub fn find_all_routes(
    graph: &DiGraph<AppInformation, CommunicationMethod>,
    current: NodeIndex,
    goal: Option<NodeIndex>, // Make goal optional
    desired_route: Option<&str>,
    path: &mut Vec<(NodeIndex, NodeIndex, CommunicationMethod)>,
    all_routes: &mut Vec<Vec<(NodeIndex, NodeIndex, CommunicationMethod)>>,
    visited: &mut HashSet<NodeIndex>,
) {
    info!("Visiting node: {:?}", graph[current].name()); // Add this line for debugging

    // If goal is provided and we've reached the goal, store the path and exit recursion
    if let Some(goal_node) = goal {
        if current == goal_node {
            all_routes.push(path.clone());
            info!("Found valid route: {:?}", path);
            return;
        }
    }

    // Mark the current node as visited
    visited.insert(current);

    // Iterate through the outgoing edges from the current node
    for edge in graph.edges(current) {
        let target_node = edge.target();
        let communication_method = edge.weight();

        // Log details about the edge being explored
        info!(
            "Exploring edge from {} to {} via {}",
            graph[current].name(),
            graph[target_node].name(),
            communication_method.communication_type()
        );

        // Skip if the target node has already been visited
        if visited.contains(&target_node) {
            info!("Node {} already visited, skipping", graph[target_node].name());
            continue;
        }

        // Proceed based on whether desired_route is Some or None
        if let Some(desired) = desired_route {
            if communication_method.route_names().contains(&desired.to_string()) {
                info!(
                    "Edge matches desired route: {} -> {} via {}",
                    graph[current].name(),
                    graph[target_node].name(),
                    communication_method.communication_type()
                );

                // Add the current edge to the path
                path.push((current, target_node, communication_method.clone()));

                // Recursively search from the target node
                find_all_routes(graph, target_node, goal, Some(desired), path, all_routes, visited);

                // Backtrack to explore other possible paths
                path.pop();
            } else {
                info!(
                    "Edge does not match desired route: {} -> {} via {}",
                    graph[current].name(),
                    graph[target_node].name(),
                    communication_method.communication_type()
                );
            }
        }
    }

    // Unmark the current node after backtracking
    visited.remove(&current);
}
pub fn find_all_routes_from_anywhere(
    graph: &DiGraph<AppInformation, CommunicationMethod>,
    desired_route: &str,
    all_routes: &mut Vec<Vec<(NodeIndex, NodeIndex, CommunicationMethod)>>,
) {
    // Iterate through all nodes in the graph and try to find routes from each
    for node_idx in graph.node_indices() {
        let mut visited = HashSet::new(); // Reset visited set for each starting node
        let mut path = Vec::new(); // Track current path

        find_all_routes(
            graph,
            node_idx,
            None, // No specific goal, search entire graph
            Some(desired_route),
            &mut path,
            all_routes,
            &mut visited,
        );
    }
}

// Function to collect and return all routes in Mermaid graph format
pub fn collect_mermaid_routes(
    graph: &DiGraph<AppInformation, CommunicationMethod>,
    routes: &Vec<Vec<(NodeIndex, NodeIndex, CommunicationMethod)>>,
) -> String {
    let mut mermaid_output = String::new();

    // Start the Mermaid graph
    mermaid_output.push_str("```mermaid\n");
    mermaid_output.push_str("graph TD\n");

    for route in routes {
        for (source, target, method) in route {
            let source_name = graph[*source].name();
            let target_name = graph[*target].name();
            let communication_type = match method.communication_type().as_str() {
                "RESTAPI" => "REST API".to_string(), // Format RESTAPI to "REST API"
                other => other.to_string(),
            };

            // Detailed communication info based on the method type
            let details = match method {
                CommunicationMethod::RESTAPI { method, endpoint, .. } => {
                    format!("{} - Method: {} - Endpoint: {}", communication_type, method, endpoint)
                }
                CommunicationMethod::Kafka { topic, .. } => {
                    format!("{} - Topic: {}", communication_type, topic)
                }
                CommunicationMethod::MQ { queue_name, .. } => {
                    format!("{} - Queue: {}", communication_type, queue_name)
                }
                CommunicationMethod::GRPC { service_name, .. } => {
                    format!("{} - Service: {}", communication_type, service_name)
                }
                CommunicationMethod::FileTransfer { file_path, .. } => {
                    format!("{} - File: {}", communication_type, file_path)
                }
                CommunicationMethod::Soap { wsdl_url, .. } => {
                    format!("{} - WSDL: {}", communication_type, wsdl_url)
                }
            };
            // Append the formatted hop in Mermaid syntax with detailed information
            mermaid_output.push_str(&format!("    {} -->|{}| {}\n", source_name, details, target_name));
        }
    }

    // End the Mermaid graph
    mermaid_output.push_str("```");

    mermaid_output
}


#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::DiGraph;
    use std::collections::HashSet;

    fn setup_test_graph() -> DiGraph<AppInformation, CommunicationMethod> {
        let mut graph = DiGraph::<AppInformation, CommunicationMethod>::new();

        let app_a = graph.add_node(AppInformation::new("AppA".to_string()));
        let app_b = graph.add_node(AppInformation::new("AppB".to_string()));
        let app_c = graph.add_node(AppInformation::new("AppC".to_string()));
        let app_d = graph.add_node(AppInformation::new("AppD".to_string()));

        // App A calls App B via REST API
        graph.add_edge(
            app_a,
            app_b,
            CommunicationMethod::RESTAPI {
                method: "GET".to_string(),
                endpoint: "/api/data".to_string(),
                route_names: vec!["payment_route".to_string()], // Ensure route name is "payment_route"
            },
        );

        // App B calls App C via Kafka
        graph.add_edge(
            app_b,
            app_c,
            CommunicationMethod::Kafka {
                topic: "topic1".to_string(),
                route_names: vec!["payment_route".to_string()], // Ensure route name is "payment_route"
            },
        );

        // App C calls App D via Kafka
        graph.add_edge(
            app_c,
            app_d,
            CommunicationMethod::Kafka {
                topic: "topic2".to_string(),
                route_names: vec!["payment_route".to_string()], // Ensure route name is "payment_route"
            },
        );

        graph
    }
    #[test]
    fn test_find_route_with_start_and_goal() {
        let graph = setup_test_graph();
        let app_a = graph.node_indices().find(|i| graph[*i].name() == "AppA").unwrap();
        let app_d = graph.node_indices().find(|i| graph[*i].name() == "AppD").unwrap();
        let desired_route = Some("payment_route");

        let mut path = Vec::new();
        let mut all_routes = Vec::new();
        let mut visited = HashSet::new();

        find_all_routes(
            &graph,
            app_a,
            Some(app_d),
            desired_route,
            &mut path,
            &mut all_routes,
            &mut visited,
        );

        assert_eq!(all_routes.len(), 1); // Expect 1 route from AppA to AppD
    }

    #[test]
    fn test_find_all_routes_by_route_name() {
        let graph = setup_test_graph();
        let desired_route = "payment_route";
        let mut all_routes = Vec::new();

        // No start and goal, search all routes matching the desired route
        find_all_routes_from_anywhere(&graph, desired_route, &mut all_routes);

        assert_eq!(all_routes.len(), 3); // Expect 3 total routes in the graph matching the "payment_route"
    }

    #[test]
    fn test_collect_mermaid_output() {
        let graph = setup_test_graph();
        let app_a = graph.node_indices().find(|i| graph[*i].name() == "AppA").unwrap();
        let app_d = graph.node_indices().find(|i| graph[*i].name() == "AppD").unwrap();
        let desired_route = Some("payment_route");

        let mut path = Vec::new();
        let mut all_routes = Vec::new();
        let mut visited = HashSet::new();

        // Find routes from AppA to AppD
        find_all_routes(
            &graph,
            app_a,
            Some(app_d),
            desired_route,
            &mut path,
            &mut all_routes,
            &mut visited,
        );

        let mermaid_output = collect_mermaid_routes(&graph, &all_routes);
        println!("{}", mermaid_output);

        assert!(mermaid_output.contains("AppA -->|REST API - Method: GET - Endpoint: /api/data| AppB"));
        assert!(mermaid_output.contains("AppB -->|Kafka - Topic: topic1| AppC"));
        assert!(mermaid_output.contains("AppC -->|Kafka - Topic: topic2| AppD"));
    }

    #[test]
    #[should_panic(expected = "desired_route must be provided when both start and goal are None")]
    fn test_no_start_no_goal_without_route() {
        let graph = setup_test_graph();
        let desired_route: Option<&str> = None;

        let mut all_routes = Vec::new();

        // Try searching with no start, no goal, and no route name, which should panic
        if desired_route.is_none() {
            panic!("desired_route must be provided when both start and goal are None");
        }

        find_all_routes_from_anywhere(&graph, desired_route.unwrap(), &mut all_routes);
    }
}