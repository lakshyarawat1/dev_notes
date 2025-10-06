# Cloud Native concepts

## Introduction

- Concept of building and running applications to take advantage of the distributed computing offered by the cloud delivery model.
- Like Scaling, Elasticity, resiliency, and flexibilty of cloud
- Created in cloud from scratch

### Why Cloud native

- Enhanced customer experience
- Facilitates frequent changes
- Can robustly develop and deliver solutions
- Written, tested and deployed on the cloud that reduces latency, and downtime
- Management and monitoring tools are implemented
- Allows ease of management
- Better security measures and analytics

### Cloud Enabled

- Application that has been moved to the cloud but was originally developed for conventional use.

### Cloud Based

- Middleground between native and enabled
- Made on in-house servers, hence do not have multi-tenant instances
- Requires manual upgrades
- Generally costlier
- Slower to deploy because of hardware provisioning or software setup

## Key Pillars

### CNCF

- Cloud Native Computing Foundation
- Open-source software foundation dedicated to making cloud-native computing universal and sustainable.
- Oracle is platinum member of CNCF. 

### Microservices

- First Pillar of Cloud native
- Designs the application as a collection of loosely coupled services.

### Containers

- Packages of software that contain all the necessary elements to run in any environment
- Microservices are packaged in containers.

### DevOps

- Combination of development and operations brought together to create a unified infrastructure designed to maximize productivity

### CI/CD

- Continuous Integration/Continuous Deployment 
- Pipelines automates yours software delivery process. 
- This helps developers merge and test code more frequently

### Service Mesh

- A configurable, low-latency infrastructure layer that controls the interaction between a network of microservices.

## Benefits of Cloud Native Development

- Faster release
- Ease of management
- Reduced cost
- Reliable system
- Avoid vendor lock-in
- Scalability
- Auto-provisioning

## Challanges

- Complex architecture
- Latest technology enhancements
- Continuous innovation
- Over-dependence on a platform or provider
- Skills shortage
- Security
- High-operational and tech cost

## Microservices Architecture

- Microservices are used to design an application that is :
    - Multi-lingual
    - Loosely coupled with other services
    - Easy to maintain and independently deployable
    - Easily scalable and highly available
    - Failure resistant

- Multiple microservices communicate with each other using either synchronous protocols (HTTP, HTTPS, gRPC) or asynchronous (AMQP, like Kafka) protocols.

## Methodology of microservices design

12 factor methodology created by heroku in 2011

- Codebase : App should have their own seperate repository and it should not be sharable with others
- Dependencies 
- Configuration : Externalize the configurations in a seperate files
- Backing services : Shifts the application from one provider to another without any modifications in the codebase.
- Build, release and run : Strictly seperate build and run stages, using CI/CD.
- Processes 
- Port Binding 
- Concurrency : Horizontal scaling instead of vertical.
- Disposability : old instances should not be impacted when new ones are created.
- Development and production parity
- Logs
- Admin processes : should be seperated
