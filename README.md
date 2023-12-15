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

## Development

### Adding a New Device Simulator
(TBD - Describe steps to add and integrate a new type of IoT device simulator.)

### Extending the Server
(TBD - Instructions on how to extend the server functionality or add new features.)

## Testing
(TBD - Guidelines on how to run unit tests and integration tests.)

## Contributing
(If this is an open project, provide instructions for how others can contribute to the project.)

## License
(Indicate the license under which this project is released, if applicable.)

## Acknowledgments
(Credits to any third-party assets, libraries, or contributors.)

## Contact
(Your contact information or links to your social media.)




