## How we built it

Bulto was built with a vision to empower SMEs in Africa by streamlining the process of securing financial assistance. The platform was architected to integrate seamlessly with banks and microfinance institutions, providing a robust and user-friendly interface for loan applications and financial management.

### Technology Stack
We chose Rust for its performance and safety guarantees to build the backend services. Rust's powerful type system and ownership model allowed us to develop a highly concurrent and memory-safe application without the typical overhead of garbage collection found in other languages.

### Microservices Architecture
We adopted a microservices architecture to ensure scalability and maintainability. Each service is responsible for a specific piece of business logic, such as user management, credit scoring, and transaction processing. This approach allowed us to deploy updates with minimal downtime and scale individual services according to demand.

### GraphQL API
We exposed our services through a GraphQL API, which provided a flexible and efficient way for front-end applications to interact with the backend. GraphQL's ability to fetch complex data structures in a single request reduced network overhead and improved the user experience.

### External Services Integration
We intend to build adapters to connect with external APIs from banks and financial institutions. These adapters handle authentication, data transformation, and error handling, providing a clean interface for our services to initiate transactions and retrieve financial data.

