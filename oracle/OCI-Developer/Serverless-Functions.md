# Serverless functions

## OCI Functions

- Function as a service
- Container native
- Open source and secure

### Benefits

- Uses a pay for execution, not for idle time.
- Auto-scales, no server provision, manage
- Event driver, and automatic triggering
- Packaged as container images

### Concepts

- Function Developers : OCI users that create and deploy functions using OCI functions
- Applications : Logical grouping of functions, common context, specifies the subnets and enable logging, isolates multiple executions
- Functions : designed to perform one specific task efficiently
    - Organized into applications
    - Stored as container images
    - Can be invoked using a CLI command or a signed HTTP request.
- Invokations : 
    - Invoked using Fn project CLI, OCI SDKs, signed HTTP requests, and other OCI services.
    - Each function has a unique invoke endpoint for signed HTTP request.
    - On first invokation, OCI functions pulls the container image, runs it as a container, and executes the function.
    - Subsequent requests are directed to the existing container.
    - Container is removed after a prolonged period of inactivity.
- Triggers : 
    - Actions elsewhere in the system that sends requests to invoke function in OCI functions.
    - Functions can have no triggers, single or multiple triggers also.

### Function Development Kits (FDKs)

- Set of helper libraries that handle system internals for containers.
- Supports Java, C#, NodeJS, Python, Go, Ruby
- Each language SDK has three components :
    - Build time base image: Contains language-specific libraries and tools to build executable functions.
    - Runtime base image : For runtime executables
    - FDK library
- Initialize function using `fn init` command. 

## Managing functions

- OCI functions architecture has two vital planes : 
    - Control plane : handles data executions
    - Data plane
- Distributed architecture

## Scheduling OCI functions

- Create a schedule : First, create a schedule using the Resource Scheduler service
- Specify functions : Select the function you wish to schedule within the schedule settings.
- Configure schedule : Define the timing, frequency, and duration to control when and how often the functions will execute
- Permissions : Create a dynamic group that includes the resource scheduler and a policy to allow it to access the function.

## Pre-built functions

- Ready to use tasks/actions
- Default creation of dynamic group and IAM policy

## Function image security

- Scanning function images
- OCI vuln scanning service is used to scan
- Signing function images with master encrpytion keys
- 