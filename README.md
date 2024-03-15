# IoTerra
### Rust-Based IoT Device Simulator

## Overview
This Rust-based IoT Device Simulator is designed to simulate interactions between various IoT devices and a server using WebSockets for real-time communication. The project is built with Actix-Web and Serde in Rust, focusing on efficient and realistic simulation of IoT environments.

![System Design Diagram](docs/IoTerraSystemDesignDOc.png)

## Features
- Simulated IoT devices including a Temperature Sensor, GPS Location Tracker, and Battery Level Monitor.
- Real-time data transmission and control using WebSockets.
- Server-side logic to handle WebSocket connections and simulate different device scenarios.
- Client dashboard for monitoring simulated IoT devices (yet to be developed).

## Getting Started

### Prerequisites
- Rust and Cargo (latest stable version)
- WebSocket client for testing (can be a web browser or dedicated WebSocket testing tools)

### Installation
1. Clone the repository:
 ```git clone https://github.com/dmarcr1997/IoTerra```
2. Navigate to the project directory:
```cd IoTerra```


### Running the Server
1. To start the server, run:
```cargo run```
2. The server will start on `localhost:8080`.

## Usage
- **Connecting a Device Simulator:** 
- Simulated IoT devices can connect to the server via WebSocket at `ws://localhost:8080/ws/?deviceId=deviceId&scenario=scenarioNumber`.
- **Client Dashboard Connection:** 
- The client dashboard can connect to the WebSocket server to receive real-time updates from the simulated devices (dashboard implementation pending).
(Your contact information or links to your social media.)




