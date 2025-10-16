# Identity Access and Resource Setup

## Resource

- An entity you can create, manage, or use within the cloud environment.
- Two types of resources :
  - Service-level resources like VMs, storage, or databases.
  - Account-level resources like organizations, folders and projects, i.e. directories, logical partitions.

### Resource Hierarchy

- Resource : Any service-level resources that are used to process your workloads.
- Resource Management : Configuring and managing and granting access to cloud resources for your team. Setup or organization of the account-level resources.
- Domain : Primary identity of the organization.
  - Linked with either Google workspace or Cloud identity account
  - A google workspace or cloud identity can only have one org
- Organization : root node of the Google Cloud hierarchy of resources
  - Defines the settings, permissions and policies of all projects, folders, resources and Billing accounts.
  - Associated with exactly one domain.
- Folders : logical grouping of projects and/or other folders.
- Projects : logical grouping of service-level resources
  - A service-level resource can only belong to a single project.

- Hierachy architectures include :
  - Environment oriented
  - Function-oriented
  - Granular access-oriented

## Architectural Hierarchy

### Environment-oriented hierarchy

- One organization that contains one folder per environment, simple to implement.
- Can pose challanges if you have to deploy services that are shared by multiple environments.

### Function-oriented hierarchy

- One organization that contains one folder per business function
- Each business function folder can contain multiple environments folders
- More flexible
- Allows shared services

### Granular-access-oriented hierarchy

- One organization contains one folder per business unit.
- Each business unit folder can contain one folder per business function
- Most flexible and extensible option

## Projects and Folders

- Project is a logical grouping of resources.
- A resource must belong to a project.
- Each project has :
  - A project name
  - Project Id
  - Project number
- Folders allows you to logical group multiple projects that share common IAM permissions.
- Commonly used to isolate projects for different departments or for different environments.

## Organizational Policies

- Allows you to apply constraints across your entire resource hierarchy.
- They are called constraints
- IAM policies manages "who has access", Org. policies manages "what has access".
- For constraints, you need to apply an enforcement
- Enforcement is made up of :
  - Resource type
  - Enforcement Method : When to enforce i.e. on create, update, delete etc.
  - Condition : CEL expression to define the logical rules of access.
  - Action : Allow or Deny

- Constraint type can be :
  - Managed : Created by GCP
  - Custom : Created by user.

## Principle of Least privilege

- Security practice of granting users,programs or processes the minimum permissions.

## Cloud IAM

- Controls who has specific permissions to access Google Cloud resources.
- Components include :
  - Principal identifier : Individuals or systems accessing the resources
  - Roles : Defines what actions(permissions) a member can perform. Like, viewer, editor or custom roles
  - Policies : Roles that connect members (who) with roles(permissions) for specific resources.

### Cloud IAM principals

- Identifies who gets access to the resources.
- Types of principals include :
  - Google Accounts : Users with google associated email ids. `user:email@gmail.com` user is the principal type
  - Service accounts : A google service instead of an individual user. `serviceAccount:my-service@project.iam.gserviceaccount.com`
  - Google groups
  - Google workspace accounts
  - Cloud identity domains
  - allAuthenticatedUsers : Special indentifier that represents all users and service accounts of users that are authenticated
  - allUsers : special identifier that represents anyone on the internet from anywhere.
  - One or more federated identities in a workforce identity pool
  - One or more federated identities in a workload identity pool
  - A set of google kubernetes engine pods
  - Resource manager principal sets (for principal access boundary policy bindings only)
  
### Roles

- Used to group permissions to define what actions users or services can perform in Google Cloud.
- Permissions cannot be assigned to users directly, instead, roles are assigned to users, granting them permissions contained in the role.
- Three types of roles :
  - Basic : Legacy roles historically used in Google Cloud, like Owner, Editor etc.
  - Predefined : Fine-grained roles managed by Google, designed for specific job functions.
  - Custom : Created by users with specific permissions for their needs.

### Permissions

- Specify what actions users or services can perform on a resource.
- Permissions correspond one-to-one with REST API methods.
- Meaning permissions directly map to actions you can perform using REST APIs request for Google Cloud Services.

### Conditions

- A rule that specifies when access to a resource is granted based on a logical expression.
- Conditions enable fine-grained, attribute-based access control for Google Cloud resources.
- Grant access only if certain attributes e.g. request time, IP address, or resource tags match the defined criteria
- Grants access only if the expression is evaluated to be true.

### Metadata

- Provides information for versioning, concurrency control, and auditing.
- It includes etags for consistent updates, version for schema specification, and optional auditConfig for logging.
- Etags enable concurrency control for consistent policy updates. Prevents race conditions during policy updates.
- Version defines the schema version to ensure consistency and compatibility across updates.

### Audit Config

- Specifies which permission types are logged and identifies any exemptions for certain identities in IAM policies.
- Audit logs types :
  - Admin Activity Logs : Records administrative actions like creating or deleting resources.
  - Data access logs : Tracks access to resources data (read/write operations).
  - Policy Denied logs : Captures attempts to access resources that were blocked by policies.

### Policy Limitations

- Each resource can have only one IAM policy.
- Policies support upto 1500 members or 250 Google groups.
- Policy changes take upto 7 minutes to propogate
- Upto 100 conditional role bindings per policy.
- Conditional roles allow upto 12 logical operations for fine-grained access.

### Conditions Limitations

- Only supported for specific services.
- Primitive roles are not supported.
- Cannot use allUsers or allAuthenticatedUsers as members.
- Upto 100 conditional bindings per condition.
- Max 20 bindings for the same role and member.

## Service Accounts

- Enables apps or VMs to securely access Google Cloud resources without user involvement.
- Used by apps or VMs to authenticate with GCP services.
- Acts as identity of a service, with permissions controlling resource access.
- Identified by a unique email address.
- Eliminates direct user involvement in authentication.
- Three types of service accounts :
  - Used managed : Created manually by users for custom applications or workloads. User controls the naming and permissions.
  - Google managed : Automatically created and managed by Google, some are visible for specific use cases, others are hidden for backend system tasks. Names typically include "Service Agent" or "Service Account".
  - Default : Created by GCP when you enable specific services like Compute Engine, App Engine etc. Automatically granted project-wide Editor role unless modified.
  