# vaex visualization

```mermaid
graph TD
    A[RabbitMQ Topic Subscriber] --> B{New Document Received?}

    B -- Yes --> C[Check Document Type]
    C -- Start --> D[Initialize Buffer for Experiment]
    C -- Not Start --> E{Is this the "End Document"?}
    E -- Yes --> F[Store Data in Postgres]
    E -- No --> G[Stream Data via WebSocket]

    F --> G
    G --> H[Emit WebSocket Events]

    H --> I[Bytewax Dataflow]
    I --> J{StreamResource Event with Image Path?}

    J -- Yes --> K[Start Vaex Server on New Port]
    K --> L[Pass Image Data to Vaex]

    L --> M[REST API Gateway]
    M --> N[Postgres for Past Messages]
    M --> O[Vaex Server for Plotly.js Visualizations]

    J -- No --> I
```
