## Inspiration
Working in Lusaka has helped us realize how hard it is to get stuff done as an SME, hard to get a bank account, hard to get funding, this makes it really hard to grow and expand small businesses, over the past 3 years we have had to bootstrap most of our funding, with no track record of sales or inquiries.
## What it does
Bulto makes it easy for small businesses to be able to borrow loans, we work with banks, Microfinance Ins
## How we built it

Bulto was built with a vision to empower SMEs in Lusaka by streamlining the process of securing financial assistance. The platform was architected to integrate seamlessly with banks and microfinance institutions, providing a robust and user-friendly interface for loan applications and financial management.

### Technology Stack
We chose Rust for its performance and safety guarantees to build the backend services. Rust's powerful type system and ownership model allowed us to develop a highly concurrent and memory-safe application without the typical overhead of garbage collection found in other languages.

### Microservices Architecture
We adopted a microservices architecture to ensure scalability and maintainability. Each service is responsible for a specific piece of business logic, such as user management, credit scoring, and transaction processing. This approach allowed us to deploy updates with minimal downtime and scale individual services according to demand.

### GraphQL API
We exposed our services through a GraphQL API, which provided a flexible and efficient way for front-end applications to interact with the backend. GraphQL's ability to fetch complex data structures in a single request reduced network overhead and improved the user experience.

### External Services Integration
We built adapters to connect with external APIs from banks and financial institutions. These adapters handle authentication, data transformation, and error handling, providing a clean interface for our services to initiate transactions and retrieve financial data.

### Continuous Integration and Deployment
We set up CI/CD pipelines to automate testing and deployment. Every code push triggered a series of automated tests, ensuring that new changes did not introduce regressions. Upon successful testing, the code was automatically deployed to our staging environment for further validation before being promoted to production.

### Security and Compliance
Security was a top priority given the sensitive nature of financial data. We implemented industry-standard encryption, access controls, and auditing mechanisms to protect user data and comply with financial regulations.

By leveraging modern technologies and methodologies, we built Bulto to be a robust and scalable platform that meets the needs of SMEs in Lusaka and beyond. Our commitment to quality and user experience has positioned Bulto as a leading solution for financial empowerment.

## Challenges we ran into

## Accomplishments that we're proud of

## What we learned

## What's next for Bulto
